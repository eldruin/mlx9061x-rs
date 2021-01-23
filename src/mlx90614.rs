//! MLX90614-specific functions

use crate::{
    ic,
    register_access::mlx90614::{Register, DEV_ADDR},
    Error, Mlx9061x, SlaveAddr,
};
use core::marker::PhantomData;
use embedded_hal::{
    blocking::{delay::DelayMs, i2c},
    digital::v2::OutputPin,
};

impl<E, I2C> Mlx9061x<I2C, ic::Mlx90614>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
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

    /// Read the object 1 temperature in celsius degrees
    pub fn object1_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TOBJ1)?;
        let t = f32::from(t) * 0.02 - 273.15;
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
    pub fn set_emissivity<D: DelayMs<u8>>(
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
        self.write_u16_eeprom(Register::EMISSIVITY, eps as u16, delay)
    }

    /// Set the entire contents of the PWMCTRL EEPROM register.
    /// See datasheet, section 8.3.3.
    pub fn set_pwmctrl<D: DelayMs<u8>>(
        &mut self,
        word: u16,
        delay: &mut D,
    ) -> Result<(), Error<E>> {
        // todo: This would make more sense with individual settings, instead
        // todo of writing the whole register with a word.
        self.write_u16_eeprom(Register::PWMCTRL, word, delay)
    }

    /// Read the entire contents of the PWMCTRL EEPROM register.
    pub fn pwmctrl(&mut self) -> Result<u16, Error<E>> {
        // todo: See note about individual settings in `set_pwmctrl`.
        self.read_u16(Register::PWMCTRL)
    }

    /// Enter sleep mode. See datasheet, section 8.4.5.
    pub fn sleep(&mut self) -> Result<(), Error<E>> {
        // self.write_u16(0b1111_1111, 0)? // todo test this.
        self.i2c.write(self.address, &[0xFF]).map_err(Error::I2C) // todo: Test this
    }

    /// Wake from sleep mode. See datasheet, section 8.4.5.
    /// SCL pin high and then PWM/SDA pin low for at least t_(DDQ) > 33ms
    pub fn wake<P: OutputPin, D: DelayMs<u8>>(&mut self, scl: &mut P, sda: &mut P, delay: &mut D) {
        scl.set_high();
        sda.set_low();
        delay.delay_ms(35);
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
