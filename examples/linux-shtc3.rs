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
    sht.wakeup(&mut delay).unwrap();

    println!(
        "Device identifier: 0x{:02x}",
        sht.device_identifier().unwrap()
    );
    println!(
        "Raw ID register:   0b{:016b}",
        sht.raw_id_register().unwrap()
    );

    println!("\nNormal mode measurements:");
    for _ in 0..3 {
        let measurement = sht.measure(PowerMode::NormalMode, &mut delay).unwrap();
        println!(
            "  {:.2} °C | {:.2} %RH",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );
    }

    println!("\nLow power mode measurements:");
    for _ in 0..3 {
        let measurement = sht.measure(PowerMode::LowPower, &mut delay).unwrap();
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
    sht.sleep().unwrap();
    print!("-> Measure: ");
    let error = sht
        .measure_temperature(PowerMode::NormalMode, &mut delay)
        .unwrap_err();
    println!("Error: {:?}", error);
    println!("-> Wakeup");
    sht.wakeup(&mut delay).unwrap();
    print!("-> Measure: ");
    let temperature = sht
        .measure_temperature(PowerMode::NormalMode, &mut delay)
        .unwrap();
    println!("Success: {:.2} °C", temperature.as_degrees_celsius());
    println!("-> Soft reset");
    sht.reset(&mut delay).unwrap();
    print!("-> Measure: ");
    let temperature = sht
        .measure_temperature(PowerMode::NormalMode, &mut delay)
        .unwrap();
    println!("Success: {:.2} °C", temperature.as_degrees_celsius());
}
