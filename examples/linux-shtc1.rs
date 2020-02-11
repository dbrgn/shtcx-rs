//! Test driver with an SHTC1 sensor on Linux.

use linux_embedded_hal::{Delay, I2cdev};
use shtcx::{self, PowerMode};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sht = shtcx::shtc1(dev, Delay);

    println!("Starting SHTC1 tests.");
    println!();

    println!("Soft reset...");
    sht.reset().unwrap();
    println!();

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
        let measurement = sht.measure(PowerMode::NormalMode).unwrap();
        println!(
            "  {:.2} °C | {:.2} %RH",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );
    }

    println!("\nLow power mode measurements:");
    for _ in 0..3 {
        let measurement = sht.measure(PowerMode::LowPower).unwrap();
        println!(
            "  {:.2} °C | {:.2} %RH",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );
    }
}
