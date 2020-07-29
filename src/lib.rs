//! This is a platform agnostic Rust driver for the MLX90614/MLX90615
//! infrared thermometers using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read the last object temperature measurement. See: [`object1_temperature()`].
//! - Read the last ambient temperature measurement. See: [`ambient_temperature()`].
//! - Read the last raw IR measurement. See: [`raw_ir_channel1()`].
//! - Get/Set the emissivity. See: [`set_emissivity()`].
//! - Get the device ID. See: [`device_id()`].
//! - Set the device address. See: [`set_address()`].
//!
//! [`object1_temperature()`]: struct.Mlx9061x.html#method.object1_temperature
//! [`ambient_temperature()`]: struct.Mlx9061x.html#method.ambient_temperature
//! [`raw_ir_channel1()`]: struct.Mlx9061x.html#method.raw_ir_channel1
//! [`set_emissivity()`]: struct.Mlx9061x.html#method.set_emissivity
//! [`device_id()`]: struct.Mlx9061x.html#method.device_id
//! [`set_address()`]: struct.Mlx9061x.html#method.set_address
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The devices
//!
//! The MLX90614/MLX90615 are a infrared thermometers for non-contact temperature
//! measurements. Both the IR sensitive thermopile detector chip and the
//! signal conditioning ASSP are integrated in the same TO-39/TO-46 can.
//! Thanks to its low noise amplifier, 17-bit/16-bit ADC and powerful DSP unit,
//! a high accuracy and resolution of the thermometer is achieved.
//!
//! Depending on the MLX90614 model they feature a single-zone or dual-zone thermopile.
//!
//! The chips feature an 10-bit PWM and SMBus interface.
//!
//! The readout resolution is 0.01°C (MLX90614) / 0.02°C (MLX90615).
//!
//! This driver uses the SMBus interface.
//!
//! Documentation:
//! - Datasheets: [MLX90614](https://www.melexis.com/-/media/files/documents/datasheets/mlx90614-datasheet-melexis.pdf), [MLX90615](https://www.melexis.com/-/media/files/documents/datasheets/mlx90615-datasheet-melexis.pdf)
//! - [SMBus communication with MLX90614](https://www.melexis.com/-/media/files/documents/application-notes/mlx90614-smbus-communication-application-note-melexis.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read the object 1 temperature with an MLX90614
//!
//! Some models feature single-zone or dual-zone thermopiles.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mlx9061x::{Mlx9061x, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let addr = SlaveAddr::default();
//! let mut sensor = Mlx9061x::new_mlx90614(dev, addr, 5).unwrap();
//! let obj_temp = sensor.object1_temperature().unwrap_or(-1.0);
//! println!("Object temperature: {:.2}ºC", obj_temp);
//! ```
//!
//! ### Read the ambient temperature with an MLX90615
//!
//! ```no_run
//! # use linux_embedded_hal::I2cdev;
//! # use mlx9061x::{Mlx9061x, SlaveAddr};
//! #
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let addr = SlaveAddr::default();
//! let mut sensor = Mlx9061x::new_mlx90615(dev, addr, 5).unwrap();
//! let temp = sensor.ambient_temperature().unwrap_or(-1.0);
//! println!("Ambient temperature: {:.2}ºC", temp);
//! ```
//!
//! ### Get the device ID of an MLX90614
//!
//! ```no_run
//! # use linux_embedded_hal::I2cdev;
//! # use mlx9061x::{Mlx9061x, SlaveAddr};
//! #
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! # let addr = SlaveAddr::default();
//! let mut sensor = Mlx9061x::new_mlx90614(dev, addr, 5).unwrap();
//! let id = sensor.device_id().unwrap_or(0);
//! println!("ID: 0x{:x?}", id);
//! ```
//!
//! ### Set the emissivity
//!
//! This change will be permanently stored in the device EEPROM.
//!
//! ```no_run
//! use linux_embedded_hal::{I2cdev, Delay};
//! use mlx9061x::{Mlx9061x, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let addr = SlaveAddr::default();
//! let mut sensor = Mlx9061x::new_mlx90614(dev, addr, 5).unwrap();
//! let mut delay = Delay{};
//! sensor.set_emissivity(0.8, &mut delay).unwrap();
//! ```
//!
//! ### Change the device address
//!
//! This change will be permanently stored in the device EEPROM.
//!
//! ```no_run
//! use linux_embedded_hal::{I2cdev, Delay};
//! use mlx9061x::{Mlx9061x, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let addr = SlaveAddr::default();
//! let mut sensor = Mlx9061x::new_mlx90614(dev, addr, 5).unwrap();
//! let mut delay = Delay{};
//! sensor.set_address(SlaveAddr::Alternative(0x5C), &mut delay).unwrap();
//! ```

#![doc(html_root_url = "https://docs.rs/mlx9061x/0.1.0")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
mod mlx90614;
mod mlx90615;
mod types;
pub use crate::types::{ic, Error, SlaveAddr};
mod crc8;
mod register_access;
use crate::crc8::crc8;
mod common;

/// MLX90614/MLX90615 device driver
#[derive(Debug)]
pub struct Mlx9061x<I2C, IC> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    eeprom_write_delay_ms: u8,
    address: u8,
    _ic: PhantomData<IC>,
}
