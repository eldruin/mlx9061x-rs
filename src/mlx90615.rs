use crate::{
    ic,
    register_access::mlx90615::{self, Register, DEV_ADDR},
    Error, Mlx9061x, SlaveAddr,
};
use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, digital::OutputPin, i2c::I2c};

impl<E, I2C> Mlx9061x<I2C, ic::Mlx90615>
where
    I2C: I2c<Error = E>,
{
    /// Create new instance of the MLX90615 device.
    ///
    /// The slave address must match the address stored in the device EEPROM.
    /// To change it you need to connect first and then change it with `set_address()`.
    /// An invalid alternative slave address will return `Error::InvalidInputData`.
    ///
    /// When writing to the EEPROM waiting a certain amount of time is necessary.
    /// This delay is configured through the `eeprom_write_delay_ms` parameter
    /// in milliseconds.
    pub fn new_mlx90615(
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
}

impl<E, I2C> Mlx9061x<I2C, ic::Mlx90615>
where
    I2C: I2c<Error = E>,
{
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

    /// Read the object temperature in celsius degrees
    pub fn object_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TOBJ)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the object temperature in celsius degrees
    ///
    /// Note ONLY use to avoid floating-point ops, as this gives less accurate
    /// temperature readings compared to using `object_temperature()`.
    pub fn object_temperature_as_int(&mut self) -> Result<u16, Error<E>> {
        let t = self.read_u16(Register::TOBJ)?;
        let t = (t * 2) / 100 - 273;
        Ok(t)
    }

    /// Read the raw IR data
    pub fn raw_ir(&mut self) -> Result<u16, Error<E>> {
        self.read_u16(Register::RAW_IR)
    }

    /// Get emissivity epsilon
    pub fn emissivity(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_u16(Register::EMISSIVITY)?;
        Ok(f32::from(raw) / 16384.0)
    }

    /// Set emissivity epsilon [0.0-1.0]
    ///
    /// Wrong values will return `Error::InvalidInputData`.
    pub fn set_emissivity<D: DelayNs>(
        &mut self,
        epsilon: f32,
        delay: &mut D,
    ) -> Result<(), Error<E>> {
        if epsilon < 0.0 || epsilon > 1.0 {
            return Err(Error::InvalidInputData);
        }
        let eps = (epsilon * 16384.0 + 0.5) as u16;
        self.write_u16_eeprom(Register::EMISSIVITY, eps, delay)
    }

    /// Get the device ID
    pub fn device_id(&mut self) -> Result<u32, Error<E>> {
        let id0 = self.read_u16(Register::ID0)?;
        let id1 = self.read_u16(Register::ID0 + 1)?;
        Ok((u32::from(id0) << 16) | u32::from(id1))
    }
}

/// Wake device from sleep mode.
///
/// Note that this includes a 39ms delay.
pub fn wake_mlx90615<E, P: OutputPin<Error = E>, D: DelayNs>(
    scl: &mut P,
    delay: &mut D,
) -> Result<(), E> {
    scl.set_low()?;
    delay.delay_ms(u32::from(mlx90615::WAKE_DELAY_MS));
    scl.set_high()
}
