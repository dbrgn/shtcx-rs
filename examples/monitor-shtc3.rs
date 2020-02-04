//! Monitor an SHTC3 sensor on Linux in the terminal.

use std::collections::VecDeque;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use linux_embedded_hal::{Delay, I2cdev};
use shtcx::{self, PowerMode};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Widget};
use tui::{Frame, Terminal};

const SENSOR_REFRESH_DELAY: Duration = Duration::from_millis(50);
const UI_REFRESH_DELAY: Duration = Duration::from_millis(25);
const DATA_CAPACITY: usize = 100;

#[derive(Default)]
struct Data {
    capacity: usize,
    temp_normal: VecDeque<i32>,
    temp_lowpwr: VecDeque<i32>,
    humi_normal: VecDeque<i32>,
    humi_lowpwr: VecDeque<i32>,
}

impl Data {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            ..Default::default()
        }
    }

    /// Truncate data to max `capacity` datapoints.
    fn truncate(&mut self) {
        self.temp_normal.truncate(self.capacity);
        self.temp_lowpwr.truncate(self.capacity);
        self.humi_normal.truncate(self.capacity);
        self.humi_lowpwr.truncate(self.capacity);
    }
}

fn show_chart<B: Backend>(
    title: &str,
    max: (f64, &str),
    data_normal: &[(f64, f64)],
    color_normal: Color,
    data_lowpwr: &[(f64, f64)],
    color_lowpwr: Color,
    frame: &mut Frame<B>,
    area: Rect,
) {
    Chart::default()
        .block(Block::default().title(title).borders(Borders::ALL))
        .x_axis(
            Axis::<&str>::default()
                .title("X Axis")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, DATA_CAPACITY as f64]),
        )
        .y_axis(
            Axis::<&str>::default()
                .title("Y Axis")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, max.0])
                .labels(&["0", max.1]),
        )
        .datasets(&[
            Dataset::default()
                .name("Low power mode")
                .marker(Marker::Braille)
                .style(Style::default().fg(color_lowpwr))
                .data(&data_lowpwr),
            Dataset::default()
                .name("Normal mode")
                .marker(Marker::Dot)
                .style(Style::default().fg(color_normal))
                .data(data_normal),
        ])
        .render(frame, area);
}

fn main() -> Result<(), io::Error> {
    // Initialize sensor driver
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sht = shtcx::shtc3(dev, Delay);

    // Initialize terminal app
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Prepare terminal
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    // Set up stop signal
    let running = Arc::new(AtomicBool::new(true));
    let run_measurements = running.clone();
    let run_render_loop = running.clone();

    // Handle Ctrl-c
    thread::spawn(move || {
        for key in io::stdin().keys() {
            if let Ok(Key::Ctrl('c')) = key {
                running.store(false, Ordering::SeqCst);
                break;
            }
        }
    });

    // Launch measurement thread
    let data = Arc::new(Mutex::new(Data::new(DATA_CAPACITY)));
    let measurement_data = data.clone();
    thread::spawn(move || {
        while run_measurements.load(Ordering::SeqCst) {
            // Do measurements
            let normal = sht.measure(PowerMode::NormalMode).unwrap();
            let lowpwr = sht.measure(PowerMode::LowPower).unwrap();

            // Update data buffer
            let mut data = measurement_data.lock().unwrap();
            data.temp_normal
                .push_front(normal.temperature.as_millidegrees_celsius());
            data.temp_lowpwr
                .push_front(lowpwr.temperature.as_millidegrees_celsius());
            data.humi_normal
                .push_front(normal.humidity.as_millipercent());
            data.humi_lowpwr
                .push_front(lowpwr.humidity.as_millipercent());
            data.truncate();

            // Sleep
            thread::sleep(SENSOR_REFRESH_DELAY);
        }
    });

    // Render loop
    while run_render_loop.load(Ordering::SeqCst) {
        terminal
            .draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size());
                let (temp_normal, temp_lowpwr, humi_normal, humi_lowpwr) = {
                    let data = data.lock().unwrap();
                    (
                        data.temp_normal
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(i, x): (usize, &i32)| (i as f64, (*x as f64) / 1000.0))
                            .collect::<Vec<_>>(),
                        data.temp_lowpwr
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(i, x): (usize, &i32)| (i as f64, (*x as f64) / 1000.0))
                            .collect::<Vec<_>>(),
                        data.humi_normal
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(i, x): (usize, &i32)| (i as f64, (*x as f64) / 1000.0))
                            .collect::<Vec<_>>(),
                        data.humi_lowpwr
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(i, x): (usize, &i32)| (i as f64, (*x as f64) / 1000.0))
                            .collect::<Vec<_>>(),
                    )
                };
                show_chart(
                    "Temperature",
                    (50.0, "50"),
                    temp_normal.as_slice(),
                    Color::Red,
                    temp_lowpwr.as_slice(),
                    Color::Magenta,
                    &mut f,
                    chunks[0],
                );
                show_chart(
                    "Humidity",
                    (100.0, "100"),
                    humi_normal.as_slice(),
                    Color::Blue,
                    humi_lowpwr.as_slice(),
                    Color::Cyan,
                    &mut f,
                    chunks[1],
                );
            })
            .unwrap();

        thread::sleep(UI_REFRESH_DELAY);
    }

    // Reset terminal
    let _ = terminal.clear();
    let _ = terminal.show_cursor();

    Ok(())
}
