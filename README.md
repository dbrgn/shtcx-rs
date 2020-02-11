# Rust SHTCx / SHTWx Driver

[![Build status][workflow-badge]][workflow]
[![Test coverage][codecov-badge]][codecov]
[![Crates.io Version][crates-io-badge]][crates-io]
[![Crates.io Downloads][crates-io-download-badge]][crates-io-download]
![No Std][no-std-badge]

This is a platform agnostic Rust driver for the Sensirion SHTCx and SHTWx
temperature / humidity sensor series, based on the
[`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

Tested with the following sensors:

- [SHTC3](https://www.sensirion.com/shtc3/)

Docs: https://docs.rs/shtcx

## The Device

The Sensirion SHTCx series offers low-power high-precision digital temperature
and humidity sensors that communicate over the I²C bus.

The SHTWx series uses the same protocol, but in a wafer-level chip-scale
package (WLCSP).

## Status

- [x] Measure temperature and humidity
- [x] Get device identifier
- [x] Sleep / Wakeup commands
- [x] Soft reset command
- [x] Support for low power mode
- [x] CRC checks
- [x] Docs

## Examples

There are a few examples in the `examples` directory: The `linux-<target>`
example queries the sensor a few times using `linux-embedded-hal`, while the
`monitor-<target>` example implements a terminal based real-time graphical
temperature/humidity monitoring tool.

![gif](monitor.gif)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


<!-- Badges -->
[workflow]: https://github.com/dbrgn/shtcx-rs/actions?query=workflow%3ACI
[workflow-badge]: https://img.shields.io/github/workflow/status/dbrgn/shtcx-rs/CI/master
[codecov]: https://codecov.io/gh/dbrgn/shtcx-rs
[codecov-badge]: https://codecov.io/gh/dbrgn/shtcx-rs/branch/master/graph/badge.svg
[crates-io]: https://crates.io/crates/shtcx
[crates-io-badge]: https://img.shields.io/crates/v/shtcx.svg?maxAge=3600
[crates-io-download]: https://crates.io/crates/shtcx
[crates-io-download-badge]: https://img.shields.io/crates/d/shtcx.svg?maxAge=3600
[no-std-badge]: https://img.shields.io/badge/no__std-yes-blue
