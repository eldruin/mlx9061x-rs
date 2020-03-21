use crate::{ic, register_access::mlx90615::Register, Error, Mlx9061x};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

impl<I2C> Mlx9061x<I2C, ic::Mlx90615> {
    /// Create new instance of the MLX90615 device.
    pub fn new_mlx90615(i2c: I2C) -> Self {
        Mlx9061x {
            i2c,
            _ic: PhantomData,
        }
    }
}

impl<E, I2C> Mlx9061x<I2C, ic::Mlx90615>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the ambient temperature in celsius degrees
    pub fn ambient_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TA)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the object temperature in celsius degrees
    pub fn object_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TOBJ)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the raw IR data
    pub fn raw_ir(&mut self) -> Result<u16, Error<E>> {
        self.read_u16(Register::RAW_IR)
    }
}
