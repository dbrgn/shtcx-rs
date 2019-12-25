//! A platform agnostic Rust driver for the Sensirion SHTCx temperature /
//! humidity sensor series, based on the
//! [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

#![deny(unsafe_code)]
// TODO: Deny missing docs
#![cfg_attr(not(test), no_std)]

/// Whether temperature or humidity is returned first when doing a measurement.
#[derive(Debug, Copy, Clone)]
enum MeasurementOrder {
    TemperatureFirst,
    HumidityFirst,
}
use MeasurementOrder::*;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
