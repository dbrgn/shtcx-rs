//! Test driver with an SHTC3 sensor on Linux.

use linux_embedded_hal::I2cdev;
use shtcx::ShtCx;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = 0x70;  // SHTC3
    let mut sht = ShtCx::new(dev, address);

    println!("Starting SHTCx tests.");
    println!();
    println!("Device identifier: 0x{:x}", sht.device_identifier().unwrap());
    println!("Raw ID register:   0b{:b}", sht.raw_id_register().unwrap());
}
