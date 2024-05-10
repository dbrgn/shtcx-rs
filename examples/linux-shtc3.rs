//! Test driver with an SHTC3 sensor on Linux.

use linux_embedded_hal::{Delay, I2cdev};
use shtcx::{self, LowPower, PowerMode};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sht = shtcx::shtc3(dev);
    let mut delay = Delay;

    println!("Starting SHTC3 tests.");
    println!("Waking up sensor.");
    println!();
    sht.wakeup(&mut delay).expect("Wakeup failed");

    println!(
        "Device identifier: 0x{:02x}",
        sht.device_identifier()
            .expect("Failed to get device identifier")
    );
    println!(
        "Raw ID register:   0b{:016b}",
        sht.raw_id_register()
            .expect("Failed to get raw ID register")
    );

    println!("\nNormal mode measurements:");
    for _ in 0..3 {
        let measurement = sht
            .measure(PowerMode::NormalMode, &mut delay)
            .expect("Normal mode measurement failed");
        println!(
            "  {:.2} °C | {:.2} %RH",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );
    }

    println!("\nLow power mode measurements:");
    for _ in 0..3 {
        let measurement = sht
            .measure(PowerMode::LowPower, &mut delay)
            .expect("Low power mode measurement failed");
        println!(
            "  {:.2} °C | {:.2} %RH",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );
    }

    println!("\nTesting power management:");
    print!("-> Measure: ");
    let temperature = sht
        .measure_temperature(PowerMode::NormalMode, &mut delay)
        .unwrap();
    println!("Success: {:.2} °C", temperature.as_degrees_celsius());
    println!("-> Sleep");
    sht.sleep().expect("Sleep command failed");
    print!("-> Measure: ");
    let error = sht
        .measure_temperature(PowerMode::NormalMode, &mut delay)
        .unwrap_err();
    println!("Error: {:?}", error);
    println!("-> Wakeup");
    sht.wakeup(&mut delay).expect("Wakeup command failed");
    print!("-> Measure: ");
    let temperature = sht
        .measure_temperature(PowerMode::NormalMode, &mut delay)
        .unwrap();
    println!("Success: {:.2} °C", temperature.as_degrees_celsius());
    println!("-> Soft reset");
    sht.reset(&mut delay).expect("Reset command failed");
    print!("-> Measure: ");
    let temperature = sht
        .measure_temperature(PowerMode::NormalMode, &mut delay)
        .unwrap();
    println!("Success: {:.2} °C", temperature.as_degrees_celsius());
}
