# Rust MLX90614 Infrared Thermometer Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/mlx9061x.svg)](https://crates.io/crates/mlx9061x)
[![Docs](https://docs.rs/mlx9061x/badge.svg)](https://docs.rs/mlx9061x)
-->
[![Build Status](https://travis-ci.com/eldruin/mlx9061x-rs.svg?branch=master)](https://travis-ci.com/eldruin/mlx9061x-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/mlx9061x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/mlx9061x-rs?branch=master)

This is a platform agnostic Rust driver for the mlx9061x infrared thermometer
using the [`embedded-hal`] traits.

<!--TODO
This driver allows you to:
-->
<!-- TODO
[Introductory blog post]()
-->

The MLX90614 is an infrared thermometer for non-contact temperature
measurements. Both the IR sensitive thermopile detector chip and the
signal conditioning ASSP are integrated in the same TO-39 can.
Thanks to its low noise amplifier, 17-bit ADC and powerful DSP unit,
a high accuracy and resolution of the thermometer is achieved.


Documentation:
- [Datasheet](https://www.melexis.com/-/media/files/documents/datasheets/mlx90614-datasheet-melexis.pdf)
- [SMBus communication with MLX90614](https://www.melexis.com/-/media/files/documents/application-notes/mlx90614-smbus-communication-application-note-melexis.pdf)

<!--TODO
## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
```
-->

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/mlx9061x-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
