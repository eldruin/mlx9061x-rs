# Rust MLX90614/MLX90615 Non-contact Infrared Thermometer Driver

[![crates.io](https://img.shields.io/crates/v/mlx9061x.svg)](https://crates.io/crates/mlx9061x)
[![Docs](https://docs.rs/mlx9061x/badge.svg)](https://docs.rs/mlx9061x)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![Build Status](https://github.com/eldruin/mlx9061x-rs/workflows/Build/badge.svg)](https://github.com/eldruin/mlx9061x-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/mlx9061x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/mlx9061x-rs?branch=master)

This is a platform agnostic Rust driver for the MLX90614/MLX90615 infrared
thermometers using the [`embedded-hal`] traits.

This driver allows you to:

- Read the last object temperature measurement. See: `object1_temperature()`.
- Read the last ambient temperature measurement. See: `ambient_temperature()`.
- Read the last raw IR measurement. See: `raw_ir_channel1()`.
- Get/Set the emissivity. See: `set_emissivity()`.
- Get the device ID. See: `device_id()`.
- Set the device address. See: `set_address()`.
- Put the device to sleep. See: `sleep()`.
- Wake the device from sleep. See: `wake_mlx90614()`.

<!-- TODO
[Introductory blog post]()
-->

The MLX90614/MLX90615 are a infrared thermometers for non-contact temperature
measurements. Both the IR sensitive thermopile detector chip and the
signal conditioning ASSP are integrated in the same TO-39/TO-46 can.
Thanks to its low noise amplifier, 17-bit/16-bit ADC and powerful DSP unit,
a high accuracy and resolution of the thermometer is achieved.

Depending on the MLX90614 model they feature a single-zone or dual-zone thermopile.

The chips feature an 10-bit PWM and SMBus interface.

The readout resolution is 0.01°C (MLX90614) / 0.02°C (MLX90615).

This driver uses the SMBus interface.

Documentation:

- Datasheets: [MLX90614](https://www.melexis.com/-/media/files/documents/datasheets/mlx90614-datasheet-melexis.pdf), [MLX90615](https://www.melexis.com/-/media/files/documents/datasheets/mlx90615-datasheet-melexis.pdf)
- [SMBus communication with MLX90614](https://www.melexis.com/-/media/files/documents/application-notes/mlx90614-smbus-communication-application-note-melexis.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use mlx9061x::{Mlx9061x, SlaveAddr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let addr = SlaveAddr::default();
    let mut sensor = Mlx9061x::new_mlx90614(dev, addr, 5).unwrap();
    loop {
        let obj_temp = sensor.object1_temperature().unwrap();
        println!("Object temperature: {:.2}ºC", obj_temp);
    }
}
```

## Features
### defmt-03

To enable [defmt](https://crates.io/crates/defmt) (version `0.3.x`) support, when specifying the dependency on `mlx9061x`, add the feature "`defmt-03`".

```toml
[dependencies]
mlx9061x = { version = "0.3.0", features = ["defmt-03"] }
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/mlx9061x-rs/issues).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
