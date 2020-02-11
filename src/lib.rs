//! # Introduction
//!
//! This is a platform agnostic Rust driver for the Sensirion SHTCx temperature /
//! humidity sensor series, based on the
//! [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.
//!
//! ## Supported Devices
//!
//! Tested with the following sensors:
//!
//! - [SHTC1](https://www.sensirion.com/shtc1/)
//! - [SHTC3](https://www.sensirion.com/shtc3/)
//!
//! The following sensors were not tested, but should work out-of-the-box:
//!
//! - [SHTW2](https://www.sensirion.com/shtw2/)
//!
//! ## Blocking / Non-Blocking Modes
//!
//! This driver currently uses only blocking calls. Non-blocking measurements may
//! be added in the future. Clock stretching is not implemented and probably won't
//! be.
//!
//! ## Examples
//!
//! There are a few examples in the `examples` directory: The `linux-<target>`
//! example queries the sensor a few times using `linux-embedded-hal`, while
//! the `monitor-<target>` example implements a terminal based real-time
//! graphical temperature/humidity monitoring tool.
//!
//! ![gif](https://raw.githubusercontent.com/dbrgn/shtcx-rs/master/monitor.gif)
//!
//! ## Usage
//!
//! ### Setup
//!
//! Instantiate a new driver instance using a [blocking I²C HAL
//! implementation](https://docs.rs/embedded-hal/0.2.*/embedded_hal/blocking/i2c/index.html)
//! and a [blocking `Delay`
//! instance](https://docs.rs/embedded-hal/0.2.*/embedded_hal/blocking/delay/index.html).
//! For example, using `linux-embedded-hal` and an SHTC3 sensor:
//!
//! ```no_run
//! use linux_embedded_hal::{Delay, I2cdev};
//! use shtcx;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sht = shtcx::shtc3(dev, Delay);
//! ```
//!
//! ### Device Info
//!
//! Then, you can query information about the sensor:
//!
//! ```no_run
//! # use linux_embedded_hal::{Delay, I2cdev};
//! # use shtcx;
//! # let mut sht = shtcx::shtc3(I2cdev::new("/dev/i2c-1").unwrap(), Delay);
//! let device_id = sht.device_identifier().unwrap();
//! let raw_id = sht.raw_id_register().unwrap();
//! ```
//!
//! ### Measurements
//!
//! For measuring your environment, you can either measure just temperature,
//! just humidity, or both:
//!
//! ```no_run
//! # use linux_embedded_hal::{Delay, I2cdev};
//! # use shtcx;
//! use shtcx::PowerMode;
//! # let mut sht = shtcx::shtc3(I2cdev::new("/dev/i2c-1").unwrap(), Delay);
//!
//! let temperature = sht.measure_temperature(PowerMode::NormalMode).unwrap();
//! let humidity = sht.measure_humidity(PowerMode::NormalMode).unwrap();
//! let combined = sht.measure(PowerMode::NormalMode).unwrap();
//!
//! println!("Temperature: {} °C", temperature.as_degrees_celsius());
//! println!("Humidity: {} %RH", humidity.as_percent());
//! println!("Combined: {} °C / {} %RH",
//!          combined.temperature.as_degrees_celsius(),
//!          combined.humidity.as_percent());
//! ```
//!
//! You can also use the low power mode for less power consumption, at the cost
//! of reduced repeatability and accuracy of the sensor signals. For more
//! information, see the ["Low Power Measurement Mode" application note][an-low-power].
//!
//! [an-low-power]: https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/2_Humidity_Sensors/Sensirion_Humidity_Sensors_SHTC3_Low_Power_Measurement_Mode.pdf
//!
//! ```no_run
//! # use linux_embedded_hal::{Delay, I2cdev};
//! # use shtcx::{self, PowerMode};
//! # let mut sht = shtcx::shtc3(I2cdev::new("/dev/i2c-1").unwrap(), Delay);
//! let measurement = sht.measure(PowerMode::LowPower).unwrap();
//! ```
//!
//! ### Low Power Mode
//!
//! Some of the sensors (e.g. the SHTC3, but not the SHTC1) support a low power
//! mode, where the sensor can be set to sleep mode when in idle state.
//!
//! For this, the `LowPower` trait needs to be imported:
//!
//! ```
//! use shtcx::LowPower;
//! ```
//!
//! Then you can send the sensor to sleep and wake it up again before
//! triggering a new measurement:
//!
//! ```no_run
//! # use linux_embedded_hal::{Delay, I2cdev};
//! # use shtcx::{self, PowerMode, LowPower};
//! # let mut sht = shtcx::shtc3(I2cdev::new("/dev/i2c-1").unwrap(), Delay);
//! sht.sleep().unwrap();
//! // ...
//! sht.wakeup().unwrap();
//! ```
//!
//! Invoking any command other than
//! [`wakeup`](trait.LowPower.html#tymethod.wakeup) while the sensor is in
//! sleep mode will result in an error.
//!
//! ### Soft Reset
//!
//! The SHTCx provides a soft reset mechanism that forces the system into a
//! well-defined state without removing the power supply. If the system is in
//! its idle state (i.e. if no measurement is in progress) the soft reset
//! command can be sent. This triggers the sensor to reset all internal state
//! machines and reload calibration data from the memory.
//!
//! ```no_run
//! # use linux_embedded_hal::{Delay, I2cdev};
//! # use shtcx::{self, PowerMode};
//! # let mut sht = shtcx::shtc3(I2cdev::new("/dev/i2c-1").unwrap(), Delay);
//! sht.reset().unwrap();
//! ```
//!
//! ### Generic Driver
//!
//! The `shtcx` driver supports use cases where the exact model of the sensor
//! is not known in advance. In that case, use the [`generic`](fn.generic.html)
//! factory function to create an instance of the driver that supports all
//! features available in all supported sensor types.
//!
//! Note however that sending commands to sensors that don't implement them
//! (e.g. sending a [`sleep`](trait.LowPower.html#tymethod.sleep)-command to an
//! SHTC1 sensor) will result in a runtime error. Furthermore, maximal timing
//! tolerances will be ensured, so using the generic driver with the SHTC3 will
//! result in slightly slower measurements (and slightly higher power
//! consumption) than when using the SHTC3 specific driver.
#![deny(unsafe_code, missing_docs)]
#![cfg_attr(not(test), no_std)]

mod crc;
mod types;

use core::marker::PhantomData;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::{Read, Write};

use crc::crc8;
pub use types::*;

/// Whether temperature or humidity is returned first when doing a measurement.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MeasurementOrder {
    TemperatureFirst,
    HumidityFirst,
}
use MeasurementOrder::*;

/// Measurement power mode: Normal mode or low power mode.
///
/// The sensors provides a low power measurement mode. Using the low power mode
/// significantly shortens the measurement duration and thus minimizes the
/// energy consumption per measurement. The benefit of ultra-low power
/// consumption comes at the cost of reduced repeatability of the sensor
/// signals: while the impact on the relative humidity signal is negligible and
/// does not affect accuracy, it has an effect on temperature accuracy.
///
/// More details can be found in the ["Low Power Measurement Mode" application
/// note][an-low-power] by Sensirion.
///
/// [an-low-power]: https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/2_Humidity_Sensors/Sensirion_Humidity_Sensors_SHTC3_Low_Power_Measurement_Mode.pdf
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PowerMode {
    /// Normal measurement.
    NormalMode,
    /// Low power measurement: Less energy consumption, but repeatability and
    /// accuracy of measurements are negatively impacted.
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
                order: TemperatureFirst,
            } => [0x78, 0x66],
            Command::Measure {
                low_power: false,
                order: HumidityFirst,
            } => [0x58, 0xE0],
            Command::Measure {
                low_power: true,
                order: TemperatureFirst,
            } => [0x60, 0x9C],
            Command::Measure {
                low_power: true,
                order: HumidityFirst,
            } => [0x40, 0x1A],
            Command::ReadIdRegister => [0xEF, 0xC8],
            Command::SoftwareReset => [0x80, 0x5D],
        }
    }
}

/// This non-public module is used to wrap public traits used inside the crate,
/// which should not be public to the user.
///
/// This helps getting around the "can't leak private trait" error message.
mod somewhat_private_traits {
    use super::PowerMode;

    pub trait MeasurementDuration {
        /// Return the max measurement duration (depending on the mode) in
        /// microseconds.
        fn max_measurement_duration(mode: PowerMode) -> u16;
    }
}
use somewhat_private_traits::*;

/// Type parameters for the different sensor classes.
pub mod sensor_class {
    /// Type parameter: First generation SHT sensor (SHTC1, SHTW2).
    pub struct Sht1Gen;
    /// Type parameter: Second generation SHT sensor (SHTC3).
    pub struct Sht2Gen;
    /// Type parameter: Generic driver that should work with all SHTCx sensors.
    pub struct ShtGeneric;
}

/// Marker trait implemented for all supported sensor classes.
pub trait ShtSensor {}
impl ShtSensor for sensor_class::Sht1Gen {}
impl ShtSensor for sensor_class::Sht2Gen {}
impl ShtSensor for sensor_class::ShtGeneric {}

/// Driver for the SHTCx sensor.
///
/// To create an instance of this, use a factory function like
/// [`shtc1`](fn.shtc1.html) or [`shtc3`](fn.shtc3.html) depending on your
/// sensor.
#[derive(Debug, Default)]
pub struct ShtCx<S: ShtSensor, I2C, D> {
    /// The chosen target sensor.
    sensor: PhantomData<S>,
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The concrete Delay implementation.
    delay: D,
    /// The I²C device address.
    address: u8,
}

/// Create a new instance of the driver for the SHTC1.
///
/// See [ShtCx](struct.ShtCx.html) for detailed documentation of the available
/// methods.
pub fn shtc1<I2C, D>(i2c: I2C, delay: D) -> ShtCx<sensor_class::Sht1Gen, I2C, D> {
    ShtCx {
        sensor: PhantomData,
        i2c,
        address: 0x70,
        delay,
    }
}

/// Create a new instance of the driver for the SHTC3.
///
/// See [ShtCx](struct.ShtCx.html) for detailed documentation of the available
/// methods.
pub fn shtc3<I2C, D>(i2c: I2C, delay: D) -> ShtCx<sensor_class::Sht2Gen, I2C, D> {
    ShtCx {
        sensor: PhantomData,
        i2c,
        address: 0x70,
        delay,
    }
}

/// Create a new instance of the driver for the SHTW2.
///
/// Since the SHTW2 is also available in an alternative address version, the
/// I²C address must be explicitly specified. For the standard SHTW2, it's 0x70.
///
/// See [ShtCx](struct.ShtCx.html) for detailed documentation of the available
/// methods.
pub fn shtw2<I2C, D>(i2c: I2C, address: u8, delay: D) -> ShtCx<sensor_class::Sht1Gen, I2C, D> {
    // Note: Internally, the SHTW2 is identical to the SHTC1, just with
    // different packaging.
    ShtCx {
        sensor: PhantomData,
        i2c,
        address,
        delay,
    }
}

/// Create a new generic instance of the driver.
///
/// See [ShtCx](struct.ShtCx.html) for detailed documentation of the available
/// methods.
pub fn generic<I2C, D>(i2c: I2C, address: u8, delay: D) -> ShtCx<sensor_class::ShtGeneric, I2C, D> {
    ShtCx {
        sensor: PhantomData,
        i2c,
        address,
        delay,
    }
}

impl MeasurementDuration for sensor_class::Sht1Gen {
    /// Return the max measurement duration in microseconds.
    ///
    /// Max measurement duration:
    /// - Normal mode: 14.4 ms (SHTC1/SHTW2 datasheet 3.1)
    /// - Low power mode: 0.94 us (SHTC1/SHTW2 low power application note)
    fn max_measurement_duration(mode: PowerMode) -> u16 {
        match mode {
            PowerMode::NormalMode => 14400,
            PowerMode::LowPower => 940,
        }
    }
}

impl MeasurementDuration for sensor_class::Sht2Gen {
    /// Return the max measurement duration (depending on the mode) in
    /// microseconds.
    ///
    /// Max measurement duration (SHTC3 datasheet 3.1):
    /// - Normal mode: 12.1 ms
    /// - Low power mode: 0.8 ms
    fn max_measurement_duration(mode: PowerMode) -> u16 {
        match mode {
            PowerMode::NormalMode => 12100,
            PowerMode::LowPower => 800,
        }
    }
}

impl MeasurementDuration for sensor_class::ShtGeneric {
    /// Return the max measurement duration (depending on the mode) in
    /// microseconds.
    ///
    /// Because this duration should work for all sensor models, it chooses the
    /// max duration of all models.
    ///
    /// Max measurement duration:
    /// - Normal mode: 14.4 ms (SHTC1, SHTW2)
    /// - Low power mode: 0.94 ms (SHTC1, SHTW2)
    fn max_measurement_duration(mode: PowerMode) -> u16 {
        match mode {
            PowerMode::NormalMode => 14400,
            PowerMode::LowPower => 940,
        }
    }
}

impl<S, I2C, D, E> ShtCx<S, I2C, D>
where
    S: ShtSensor + MeasurementDuration,
    I2C: Read<Error = E> + Write<Error = E>,
    D: DelayUs<u16> + DelayMs<u16>,
{
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
    /// Should be 0x47 (71) for the SHTC3 and 0x07 (7) for the SHTC1.
    pub fn device_identifier(&mut self) -> Result<u8, Error<E>> {
        let ident = self.raw_id_register()?;
        let lsb = (ident & 0b0011_1111) as u8;
        let msb = ((ident & 0b00001000_00000000) >> 5) as u8;
        Ok(lsb | msb)
    }

    /// Do a measurement with the specified measurement order and write the
    /// result into the provided buffer.
    ///
    /// If you just need one of the two measurements, provide a 3-byte buffer
    /// instead of a 6-byte buffer.
    fn measure_partial(
        &mut self,
        mode: PowerMode,
        order: MeasurementOrder,
        buf: &mut [u8],
    ) -> Result<(), Error<E>> {
        // Request measurement
        self.send_command(Command::Measure {
            low_power: match mode {
                PowerMode::LowPower => true,
                PowerMode::NormalMode => false,
            },
            order,
        })?;

        // Wait for measurement
        self.delay.delay_us(S::max_measurement_duration(mode));

        // Read response
        self.read_with_crc(buf)?;
        Ok(())
    }

    /// Run a temperature/humidity measurement and return the combined result.
    ///
    /// This is a blocking function call.
    pub fn measure(&mut self, mode: PowerMode) -> Result<Measurement, Error<E>> {
        let mut buf = [0; 6];
        self.measure_partial(mode, MeasurementOrder::TemperatureFirst, &mut buf)?;
        Ok(Measurement {
            temperature: Temperature::from_raw(u16::from_be_bytes([buf[0], buf[1]])),
            humidity: Humidity::from_raw(u16::from_be_bytes([buf[3], buf[4]])),
        })
    }

    /// Run a temperature measurement and return the result.
    ///
    /// This is a blocking function call.
    ///
    /// Internally, it will request a measurement in "temperature first" mode
    /// and only read the first half of the measurement response.
    pub fn measure_temperature(&mut self, mode: PowerMode) -> Result<Temperature, Error<E>> {
        let mut buf = [0; 3];
        self.measure_partial(mode, MeasurementOrder::TemperatureFirst, &mut buf)?;
        Ok(Temperature::from_raw(u16::from_be_bytes([buf[0], buf[1]])))
    }

    /// Run a humidity measurement and return the result.
    ///
    /// This is a blocking function call.
    ///
    /// Internally, it will request a measurement in "humidity first" mode
    /// and only read the first half of the measurement response.
    pub fn measure_humidity(&mut self, mode: PowerMode) -> Result<Humidity, Error<E>> {
        let mut buf = [0; 3];
        self.measure_partial(mode, MeasurementOrder::HumidityFirst, &mut buf)?;
        Ok(Humidity::from_raw(u16::from_be_bytes([buf[0], buf[1]])))
    }

    /// Trigger a soft reset.
    ///
    /// The SHTC3 provides a soft reset mechanism that forces the system into a
    /// well-defined state without removing the power supply. If the system is
    /// in its idle state (i.e. if no measurement is in progress) the soft
    /// reset command can be sent. This triggers the sensor to reset all
    /// internal state machines and reload calibration data from the memory.
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.send_command(Command::SoftwareReset)?;
        // Table 5: 180-240 µs
        self.delay.delay_us(240);
        Ok(())
    }
}

/// Low power functionality (sleep and wakeup).
///
/// This functionality is only present on some of the sensors (e.g. the SHTC3,
/// but not the SHTC1).
pub trait LowPower<E> {
    /// Set sensor to sleep mode.
    ///
    /// When in sleep mode, the sensor consumes around 0.3-0.6 µA. It requires
    /// a dedicated [`wakeup`](#method.wakeup) command to enable further I2C
    /// communication.
    fn sleep(&mut self) -> Result<(), Error<E>>;

    /// Wake up sensor from [sleep mode](#method.sleep).
    fn wakeup(&mut self) -> Result<(), Error<E>>;
}

macro_rules! impl_low_power {
    ($target:ty) => {
        impl<I2C, D, E> LowPower<E> for ShtCx<$target, I2C, D>
        where
            I2C: Read<Error = E> + Write<Error = E>,
            D: DelayUs<u16> + DelayMs<u16>,
        {
            fn sleep(&mut self) -> Result<(), Error<E>> {
                self.send_command(Command::Sleep)
            }

            fn wakeup(&mut self) -> Result<(), Error<E>> {
                self.send_command(Command::WakeUp)?;
                // Table 5: 180-240 µs
                self.delay.delay_us(240);
                Ok(())
            }
        }
    };
}

impl_low_power!(sensor_class::Sht2Gen);
impl_low_power!(sensor_class::ShtGeneric);

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::ErrorKind;

    use embedded_hal_mock::delay::MockNoop as NoopDelay;
    use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction};
    use embedded_hal_mock::MockError;

    const SHT_ADDR: u8 = 0x70;

    mod core {
        use super::*;

        /// Test whether the `send_command` function propagates I²C errors.
        #[test]
        fn send_command_error() {
            let expectations = [Transaction::write(SHT_ADDR, vec![0xef, 0xc8])
                .with_error(MockError::Io(ErrorKind::Other))];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc1(mock, NoopDelay);
            let err = sht.send_command(Command::ReadIdRegister).unwrap_err();
            assert_eq!(err, Error::I2c(MockError::Io(ErrorKind::Other)));
            sht.destroy().done();
        }

        /// Test the `validate_crc` function.
        #[test]
        fn validate_crc() {
            let mock = I2cMock::new(&[]);
            let sht = shtc3(mock, NoopDelay);

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
            let mut sht = shtc3(mock, NoopDelay);
            sht.read_with_crc(&mut buf).unwrap();
            assert_eq!(buf, [0xbe, 0xef, 0x92]);
            sht.destroy().done();

            // Invalid CRC
            let expectations = [Transaction::read(SHT_ADDR, vec![0xbe, 0xef, 0x00])];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc3(mock, NoopDelay);
            match sht.read_with_crc(&mut buf) {
                Err(Error::Crc) => {}
                Err(_) => panic!("Invalid error: Must be Crc"),
                Ok(_) => panic!("CRC check did not fail"),
            }
            assert_eq!(buf, [0xbe, 0xef, 0x00]); // Buf was changed
            sht.destroy().done();
        }
    }

    mod factory_functions {
        use super::*;

        #[test]
        fn new_shtc1() {
            let mock = I2cMock::new(&[]);
            let sht = shtc1(mock, NoopDelay);
            assert_eq!(sht.address, 0x70);
        }

        #[test]
        fn new_shtc3() {
            let mock = I2cMock::new(&[]);
            let sht = shtc3(mock, NoopDelay);
            assert_eq!(sht.address, 0x70);
        }

        #[test]
        fn new_shtw2() {
            let mock = I2cMock::new(&[]);
            let sht = shtw2(mock, 0x42, NoopDelay);
            assert_eq!(sht.address, 0x42);
        }

        #[test]
        fn new_generic() {
            let mock = I2cMock::new(&[]);
            let sht = generic(mock, 0x23, NoopDelay);
            assert_eq!(sht.address, 0x23);
        }
    }

    mod device_info {
        use super::*;

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
            let mut sht = shtc3(mock, NoopDelay);
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
            let mut sht = shtc3(mock, NoopDelay);
            let ident = sht.device_identifier().unwrap();
            assert_eq!(ident, 0b01000111);
            sht.destroy().done();
        }
    }

    mod measurements {
        use super::*;

        #[test]
        fn measure_normal() {
            let expectations = [
                // Expect a write command: Normal mode measurement, temperature
                // first, no clock stretching.
                Transaction::write(SHT_ADDR, vec![0x78, 0x66]),
                // Return the measurement result (using example values from the
                // datasheet, section 5.4 "Measuring and Reading the Signals")
                Transaction::read(
                    SHT_ADDR,
                    vec![
                        0b0110_0100,
                        0b1000_1011,
                        0b1100_0111,
                        0b1010_0001,
                        0b0011_0011,
                        0b0001_1100,
                    ],
                ),
            ];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc1(mock, NoopDelay);
            let measurement = sht.measure(PowerMode::NormalMode).unwrap();
            assert_eq!(measurement.temperature.as_millidegrees_celsius(), 23_730); // 23.7°C
            assert_eq!(measurement.humidity.as_millipercent(), 62_968); // 62.9 %RH
            sht.destroy().done();
        }

        #[test]
        fn measure_low_power() {
            let expectations = [
                // Expect a write command: Low power mode measurement, temperature
                // first, no clock stretching.
                Transaction::write(SHT_ADDR, vec![0x60, 0x9C]),
                // Return the measurement result (using example values from the
                // datasheet, section 5.4 "Measuring and Reading the Signals")
                Transaction::read(
                    SHT_ADDR,
                    vec![
                        0b0110_0100,
                        0b1000_1011,
                        0b1100_0111,
                        0b1010_0001,
                        0b0011_0011,
                        0b0001_1100,
                    ],
                ),
            ];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc3(mock, NoopDelay);
            let measurement = sht.measure(PowerMode::LowPower).unwrap();
            assert_eq!(measurement.temperature.as_millidegrees_celsius(), 23_730); // 23.7°C
            assert_eq!(measurement.humidity.as_millipercent(), 62_968); // 62.9 %RH
            sht.destroy().done();
        }

        #[test]
        fn measure_temperature_only() {
            let expectations = [
                // Expect a write command: Normal mode measurement, temperature
                // first, no clock stretching.
                Transaction::write(SHT_ADDR, vec![0x78, 0x66]),
                // Return the measurement result (using example values from the
                // datasheet, section 5.4 "Measuring and Reading the Signals")
                Transaction::read(SHT_ADDR, vec![0b0110_0100, 0b1000_1011, 0b1100_0111]),
            ];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc3(mock, NoopDelay);
            let temperature = sht.measure_temperature(PowerMode::NormalMode).unwrap();
            assert_eq!(temperature.as_millidegrees_celsius(), 23_730); // 23.7°C
            sht.destroy().done();
        }

        #[test]
        fn measure_humidity_only() {
            let expectations = [
                // Expect a write command: Normal mode measurement, humidity
                // first, no clock stretching.
                Transaction::write(SHT_ADDR, vec![0x58, 0xE0]),
                // Return the measurement result (using example values from the
                // datasheet, section 5.4 "Measuring and Reading the Signals")
                Transaction::read(SHT_ADDR, vec![0b1010_0001, 0b0011_0011, 0b0001_1100]),
            ];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc3(mock, NoopDelay);
            let humidity = sht.measure_humidity(PowerMode::NormalMode).unwrap();
            assert_eq!(humidity.as_millipercent(), 62_968); // 62.9 %RH
            sht.destroy().done();
        }

        /// Ensure that I²C write errors are handled when measuring.
        #[test]
        fn measure_write_error() {
            let expectations = [Transaction::write(SHT_ADDR, vec![0x60, 0x9C])
                .with_error(MockError::Io(ErrorKind::Other))];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc3(mock, NoopDelay);
            let err = sht.measure(PowerMode::LowPower).unwrap_err();
            assert_eq!(err, Error::I2c(MockError::Io(ErrorKind::Other)));
            sht.destroy().done();
        }
    }

    mod power_management {
        use super::*;

        /// Test the `sleep` function.
        #[test]
        fn sleep() {
            let expectations = [Transaction::write(SHT_ADDR, vec![0xB0, 0x98])];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc3(mock, NoopDelay);
            sht.sleep().unwrap();
            sht.destroy().done();
        }

        /// Test the `wakeup` function.
        #[test]
        fn wakeup() {
            let expectations = [Transaction::write(SHT_ADDR, vec![0x35, 0x17])];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc3(mock, NoopDelay);
            sht.wakeup().unwrap();
            sht.destroy().done();
        }

        /// Test the `reset` function.
        #[test]
        fn reset() {
            let expectations = [Transaction::write(SHT_ADDR, vec![0x80, 0x5D])];
            let mock = I2cMock::new(&expectations);
            let mut sht = shtc3(mock, NoopDelay);
            sht.reset().unwrap();
            sht.destroy().done();
        }
    }
}
