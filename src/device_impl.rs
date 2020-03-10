use crate::{ic, mlx90614, Error, Mlx9061x};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

impl<I2C> Mlx9061x<I2C, ic::Mlx90614> {
    /// Create new instance of the MLX90614 device.
    pub fn new_mlx90614(i2c: I2C) -> Self {
        Mlx9061x {
            i2c,
            _ic: PhantomData,
        }
    }
}

impl<I2C, IC> Mlx9061x<I2C, IC> {
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<E, I2C, IC> Mlx9061x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the ambient temperature in celsius degrees
    pub fn ambient_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(mlx90614::Register::TA)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the object 1 temperature in celsius degrees
    pub fn object1_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(mlx90614::Register::TOBJ1)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the object 2 temperature in celsius degrees
    ///
    /// Note that this is only available in dual-zone device variants.
    pub fn object2_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(mlx90614::Register::TOBJ2)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the channel 1 raw IR data
    pub fn raw_ir_channel1(&mut self) -> Result<u16, Error<E>> {
        self.read_u16(mlx90614::Register::RAW_IR1)
    }

    /// Read the channel 2 raw IR data
    pub fn raw_ir_channel2(&mut self) -> Result<u16, Error<E>> {
        self.read_u16(mlx90614::Register::RAW_IR2)
    }
}
