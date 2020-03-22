//! This is a platform agnostic Rust driver for the mlx9061x
//! infrared thermometer using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! <!--TODO
//! This driver allows you to:
//! -->
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The devices
//!
//! The MLX90614 is an infrared thermometer for non-contact temperature
//! measurements. Both the IR sensitive thermopile detector chip and the
//! signal conditioning ASSP are integrated in the same TO-39 can.
//! Thanks to its low noise amplifier, 17-bit ADC and powerful DSP unit,
//! a high accuracy and resolution of the thermometer is achieved.
//!
//! Documentation:
//! - [Datasheet](https://www.melexis.com/-/media/files/documents/datasheets/mlx90614-datasheet-melexis.pdf)
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

/// mlx9061x device driver
#[derive(Debug)]
pub struct Mlx9061x<I2C, D, IC> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    eeprom_write_delay_ms: u8,
    delay_ms: D,
    address: u8,
    _ic: PhantomData<IC>,
}
