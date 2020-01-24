//! Test driver with an SHTC3 sensor on Linux.

use linux_embedded_hal::{Delay, I2cdev};
use shtcx::{PowerMode, ShtCx};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = 0x70; // SHTC3
    let mut sht = ShtCx::new(dev, address, Delay);

    println!("Starting SHTCx tests.");
    println!("Waking up sensor.");
    println!();
    sht.wakeup().unwrap();

    println!(
        "Device identifier: 0x{:x}",
        sht.device_identifier().unwrap()
    );
    println!("Raw ID register:   0b{:b}", sht.raw_id_register().unwrap());

    println!("\nNormal mode measurements:");
    for _ in 0..3 {
        let measurement = sht.measure(PowerMode::NormalMode).unwrap();
        println!(
            "- {:.2} °C | {:.2} %RH",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );
    }

    println!("\nLow power mode measurements:");
    for _ in 0..3 {
        let measurement = sht.measure(PowerMode::LowPower).unwrap();
        println!(
            "- {:.2} °C | {:.2} %RH",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );
    }

    println!("\nPutting sensor to sleep...");
    sht.sleep().unwrap();

    print!("-> Measurement: ");
    let error = sht.measure_temperature(PowerMode::NormalMode).unwrap_err();
    println!("Error: {:?}", error);
    println!("\nWaking up sensor...");
    sht.wakeup().unwrap();
    print!("-> Measurement: ");
    let temperature = sht.measure_temperature(PowerMode::NormalMode).unwrap();
    println!("Success: {:.2} °C", temperature.as_degrees_celsius());
}
