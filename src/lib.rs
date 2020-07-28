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
