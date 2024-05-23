//! MLX90614-specific functions

use crate::{
    ic,
    register_access::mlx90614::{self, Register, DEV_ADDR},
    Error, Mlx9061x, SlaveAddr,
};
use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, digital::OutputPin, i2c::I2c};

impl<E, I2C> Mlx9061x<I2C, ic::Mlx90614>
where
    I2C: I2c<Error = E>,
{
    /// Create new instance of the MLX90614 device.
    ///
    /// The slave address must match the address stored in the device EEPROM.
    /// To change it you need to connect first and then change it with `set_address()`.
    /// An invalid alternative slave address will return `Error::InvalidInputData`.
    ///
    /// When writing to the EEPROM waiting a certain amount of time is necessary.
    /// This delay is configured through the `eeprom_write_delay_ms` parameter
    /// in milliseconds.
    pub fn new_mlx90614(
        i2c: I2C,
        address: SlaveAddr,
        eeprom_write_delay_ms: u8,
    ) -> Result<Self, Error<E>> {
        let address = Self::get_address(address, DEV_ADDR)?;
        Ok(Mlx9061x {
            i2c,
            eeprom_write_delay_ms,
            address,
            _ic: PhantomData,
        })
    }

    /// Read the ambient temperature in celsius degrees
    pub fn ambient_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TA)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the ambient temperature in celsius degrees as u16 value
    ///
    /// Note ONLY use to avoid floating-point ops, as this gives less accurate
    /// temperature readings compared to using `ambient_temperature()`.
    pub fn ambient_temperature_as_int(&mut self) -> Result<u16, Error<E>> {
        let t = self.read_u16(Register::TA)?;
        let t = (t * 2) / 100 - 273;
        Ok(t)
    }

    /// Read the object 1 temperature in celsius degrees
    pub fn object1_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TOBJ1)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the object 1 temperature in celsius degrees as u16 value
    ///
    /// Note ONLY use to avoid floating-point ops, as this gives less accurate
    /// temperature readings compared to using `object1_temperature()`.
    pub fn object1_temperature_as_int(&mut self) -> Result<u16, Error<E>> {
        let t = self.read_u16(Register::TOBJ1)?;
        let t = (t * 2) / 100 - 273;
        Ok(t)
    }

    /// Read the object 2 temperature in celsius degrees
    ///
    /// Note that this is only available in dual-zone thermopile device variants.
    pub fn object2_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TOBJ2)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the object 2 temperature in celsius degrees as u16 value
    ///
    /// Note that this is only available in dual-zone thermopile device variants.
    ///
    /// Note ONLY use to avoid floating-point ops, as this gives less accurate
    /// temperature readings compared to using `object2_temperature()`.
    pub fn object2_temperature_as_int(&mut self) -> Result<u16, Error<E>> {
        let t = self.read_u16(Register::TOBJ2)?;
        let t = (t * 2) / 100 - 273;
        Ok(t)
    }

    /// Read the channel 1 raw IR data
    pub fn raw_ir_channel1(&mut self) -> Result<u16, Error<E>> {
        self.read_u16(Register::RAW_IR1)
    }

    /// Read the channel 2 raw IR data
    pub fn raw_ir_channel2(&mut self) -> Result<u16, Error<E>> {
        self.read_u16(Register::RAW_IR2)
    }

    /// Get emissivity epsilon
    pub fn emissivity(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_u16(Register::EMISSIVITY)?;
        Ok(f32::from(raw) / 65535.0)
    }

    /// Set emissivity epsilon [0.1-1.0]
    ///
    /// Wrong values will return `Error::InvalidInputData`.
    pub fn set_emissivity<D: DelayNs>(
        &mut self,
        epsilon: f32,
        delay: &mut D,
    ) -> Result<(), Error<E>> {
        if epsilon < 0.1 || epsilon > 1.0 {
            return Err(Error::InvalidInputData);
        }
        let eps = (epsilon * 65535.0 + 0.5) as u16;
        if eps < 6553 {
            return Err(Error::InvalidInputData);
        }
        self.write_u16_eeprom(Register::EMISSIVITY, eps, delay)
    }

    /// Get the device ID
    pub fn device_id(&mut self) -> Result<u64, Error<E>> {
        let mut id = 0;
        for i in 0..4 {
            let part = self.read_u16(Register::ID0 + i)?;
            let part = u64::from(part) << (16 * (3 - i));
            id |= part;
        }
        Ok(id)
    }
}

/// Wake device from sleep mode.
///
/// Note that this includes a 33ms delay.
pub fn wake_mlx90614<E, SclPin: OutputPin<Error = E>, SdaPin: OutputPin<Error = E>, D: DelayNs>(
    scl: &mut SclPin,
    sda: &mut SdaPin,
    delay: &mut D,
) -> Result<(), E> {
    scl.set_high()?;
    sda.set_low()?;
    delay.delay_ms(mlx90614::WAKE_DELAY_MS as u32);
    sda.set_high()
}
