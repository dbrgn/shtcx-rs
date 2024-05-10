//! Test driver with an SHTC1 sensor on Linux.

use linux_embedded_hal::{Delay, I2cdev};
use shtcx::{self, PowerMode};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sht = shtcx::shtc1(dev);
    let mut delay = Delay;

    println!("Starting SHTC1 tests.");
    println!();

    println!("Soft reset...");
    sht.reset(&mut delay).expect("Soft reset failed");
    println!();

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
}
