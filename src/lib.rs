//! A platform agnostic Rust driver for the Sensirion SHTCx temperature /
//! humidity sensor series, based on the
//! [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

#![deny(unsafe_code)]
// TODO: Deny missing docs
#![cfg_attr(not(test), no_std)]

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

/// Whether temperature or humidity is returned first when doing a measurement.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MeasurementOrder {
    TemperatureFirst,
    HumidityFirst,
}
use MeasurementOrder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PowerMode {
    NormalMode,
    LowPower,
}

/// All possible errors in this crate
#[derive(Debug, PartialEq, Clone)]
pub enum Error<E> {
    /// I²C bus error
    I2c(E),
    /// CRC checksum validation failed
    Crc,
}

/// I²C commands sent to the sensor.
#[derive(Debug, Copy, Clone)]
enum Command {
    /// Go into sleep mode.
    Sleep,
    /// Wake up from sleep mode.
    WakeUp,
    /// Measurement commands.
    Measure {
        low_power: bool,
        clock_stretching: bool,
        order: MeasurementOrder,
    },
    /// Software reset.
    SoftwareReset,
    /// Read ID register.
    ReadIdRegister,
}

impl Command {
    fn as_bytes(self) -> [u8; 2] {
        match self {
            Command::Sleep => [0xB0, 0x98],
            Command::WakeUp => [0x35, 0x17],
            Command::Measure {
                low_power: false,
                clock_stretching: true,
                order: TemperatureFirst,
            } => [0x7C, 0xA2],
            Command::Measure {
                low_power: false,
                clock_stretching: true,
                order: HumidityFirst,
            } => [0x5C, 0x24],
            Command::Measure {
                low_power: false,
                clock_stretching: false,
                order: TemperatureFirst,
            } => [0x78, 0x66],
            Command::Measure {
                low_power: false,
                clock_stretching: false,
                order: HumidityFirst,
            } => [0x58, 0xE0],
            Command::Measure {
                low_power: true,
                clock_stretching: true,
                order: TemperatureFirst,
            } => [0x64, 0x58],
            Command::Measure {
                low_power: true,
                clock_stretching: true,
                order: HumidityFirst,
            } => [0x44, 0xDE],
            Command::Measure {
                low_power: true,
                clock_stretching: false,
                order: TemperatureFirst,
            } => [0x60, 0x9C],
            Command::Measure {
                low_power: true,
                clock_stretching: false,
                order: HumidityFirst,
            } => [0x40, 0x1A],
            Command::ReadIdRegister => [0xEF, 0xC8],
            Command::SoftwareReset => [0x80, 0x5D],
        }
    }
}

/// Driver for the SHTCx
#[derive(Debug, Default)]
pub struct ShtCx<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measurement {
    /// Raw temperature value
    temperature: u16,
    /// Raw humidity value
    humidity: u16,
}

impl<I2C, E> ShtCx<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance of the SGP30 driver.
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write an I²C command to the sensor.
    fn send_command(&mut self, command: Command) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &command.as_bytes())
            .map_err(Error::I2c)
    }

    /// Iterate over the provided buffer and validate the CRC8 checksum.
    ///
    /// If the checksum is wrong, return `Error::Crc`.
    ///
    /// Note: This method will consider every third byte a checksum byte. If
    /// the buffer size is not a multiple of 3, then not all data will be
    /// validated.
    fn validate_crc(&self, buf: &[u8]) -> Result<(), Error<E>> {
        for chunk in buf.chunks(3) {
            if chunk.len() == 3 && crc8(&[chunk[0], chunk[1]]) != chunk[2] {
                return Err(Error::Crc);
            }
        }
        Ok(())
    }

    /// Read data into the provided buffer and validate the CRC8 checksum.
    ///
    /// If the checksum is wrong, return `Error::Crc`.
    ///
    /// Note: This method will consider every third byte a checksum byte. If
    /// the buffer size is not a multiple of 3, then not all data will be
    /// validated.
    fn read_with_crc(&mut self, mut buf: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c.read(self.address, &mut buf).map_err(Error::I2c)?;
        self.validate_crc(buf)
    }

    /// Return the raw ID register.
    pub fn raw_id_register(&mut self) -> Result<u16, Error<E>> {
        // Request serial number
        self.send_command(Command::ReadIdRegister)?;

        // Read id register
        let mut buf = [0; 3];
        self.read_with_crc(&mut buf)?;

        Ok(u16::from_be_bytes([buf[0], buf[1]]))
    }

    /// Return the 7-bit device identifier.
    ///
    /// Should be 0x47 (71) for the SHTC3.
    pub fn device_identifier(&mut self) -> Result<u8, Error<E>> {
        let ident = self.raw_id_register()?;
        let lsb = (ident & 0b00111111) as u8;
        let msb = ((ident & 0b00001000_00000000) >> 5) as u8;
        Ok(lsb | msb)
    }

    pub fn measure(&mut self, mode: PowerMode) -> Result<Measurement, Error<E>> {
        // Request measurement 
        self.send_command(Command::Measure {
            low_power: false,
            clock_stretching: false,
            order: MeasurementOrder::TemperatureFirst,
        })?;

        // Read response
        let mut buf = [0; 6];
        self.read_with_crc(&mut buf)?;
        Ok(Measurement {
            temperature: ((buf[0] as u16) << 8) & buf[1] as u16,
            humidity: ((buf[2] as u16) << 8) & buf[3] as u16,
        })
    }
}

/// Calculate the CRC8 checksum.
///
/// Implementation based on the reference implementation by Sensirion.
fn crc8(data: &[u8]) -> u8 {
    const CRC8_POLYNOMIAL: u8 = 0x31;
    let mut crc: u8 = 0xff;
    for byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) > 0 {
                crc = (crc << 1) ^ CRC8_POLYNOMIAL;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::ErrorKind;

    use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction};
    use embedded_hal_mock::MockError;

    const SHT_ADDR: u8 = 0x70;

    /// Test whether the `send_command` function propagates I²C errors.
    #[test]
    fn send_command_error() {
        let mock = I2cMock::new(&[Transaction::write(SHT_ADDR, vec![0xef, 0xc8])
            .with_error(MockError::Io(ErrorKind::Other))]);
        let mut sht = ShtCx::new(mock, SHT_ADDR);
        let err = sht.send_command(Command::ReadIdRegister).unwrap_err();
        assert_eq!(err, Error::I2c(MockError::Io(ErrorKind::Other)));
        sht.destroy().done();
    }

    /// Test the crc8 function against the test value provided in the
    /// SHTC3 datasheet (section 5.10).
    #[test]
    fn crc8_test_value() {
        assert_eq!(crc8(&[0x00]), 0xac);
        assert_eq!(crc8(&[0xbe, 0xef]), 0x92);
    }

    /// Test the `validate_crc` function.
    #[test]
    fn validate_crc() {
        let mock = I2cMock::new(&[]);
        let sht = ShtCx::new(mock, SHT_ADDR);

        // Not enough data
        sht.validate_crc(&[]).unwrap();
        sht.validate_crc(&[0xbe]).unwrap();
        sht.validate_crc(&[0xbe, 0xef]).unwrap();

        // Valid CRC
        sht.validate_crc(&[0xbe, 0xef, 0x92]).unwrap();

        // Invalid CRC
        match sht.validate_crc(&[0xbe, 0xef, 0x91]) {
            Err(Error::Crc) => {}
            Err(_) => panic!("Invalid error: Must be Crc"),
            Ok(_) => panic!("CRC check did not fail"),
        }

        // Valid CRC (8 bytes)
        sht.validate_crc(&[0xbe, 0xef, 0x92, 0xbe, 0xef, 0x92, 0x00, 0x00])
            .unwrap();

        // Invalid CRC (8 bytes)
        match sht.validate_crc(&[0xbe, 0xef, 0x92, 0xbe, 0xef, 0xff, 0x00, 0x00]) {
            Err(Error::Crc) => {}
            Err(_) => panic!("Invalid error: Must be Crc"),
            Ok(_) => panic!("CRC check did not fail"),
        }

        sht.destroy().done();
    }

    /// Test the `read_with_crc` function.
    #[test]
    fn read_with_crc() {
        let mut buf = [0; 3];

        // Valid CRC
        let expectations = [Transaction::read(SHT_ADDR, vec![0xbe, 0xef, 0x92])];
        let mock = I2cMock::new(&expectations);
        let mut sht = ShtCx::new(mock, SHT_ADDR);
        sht.read_with_crc(&mut buf).unwrap();
        assert_eq!(buf, [0xbe, 0xef, 0x92]);
        sht.destroy().done();

        // Invalid CRC
        let expectations = [Transaction::read(SHT_ADDR, vec![0xbe, 0xef, 0x00])];
        let mock = I2cMock::new(&expectations);
        let mut sgp = ShtCx::new(mock, SHT_ADDR);
        match sgp.read_with_crc(&mut buf) {
            Err(Error::Crc) => {}
            Err(_) => panic!("Invalid error: Must be Crc"),
            Ok(_) => panic!("CRC check did not fail"),
        }
        assert_eq!(buf, [0xbe, 0xef, 0x00]); // Buf was changed
        sgp.destroy().done();
    }

    /// Test the `raw_id_register` function.
    #[test]
    fn raw_id_register() {
        let msb = 0b00001000;
        let lsb = 0b00000111;
        let crc = crc8(&[msb, lsb]);
        let expectations = [
            Transaction::write(SHT_ADDR, vec![0xef, 0xc8]),
            Transaction::read(SHT_ADDR, vec![msb, lsb, crc]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sht = ShtCx::new(mock, SHT_ADDR);
        let val = sht.raw_id_register().unwrap();
        assert_eq!(val, (msb as u16) << 8 | (lsb as u16));
        sht.destroy().done();
    }

    /// Test the `device_identifier` function.
    #[test]
    fn device_identifier() {
        let msb = 0b00001000;
        let lsb = 0b00000111;
        let crc = crc8(&[msb, lsb]);
        let expectations = [
            Transaction::write(SHT_ADDR, vec![0xef, 0xc8]),
            Transaction::read(SHT_ADDR, vec![msb, lsb, crc]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sht = ShtCx::new(mock, SHT_ADDR);
        let ident = sht.device_identifier().unwrap();
        assert_eq!(ident, 0b01000111);
        sht.destroy().done();
    }
}
