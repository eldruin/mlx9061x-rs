use crate::{ic, mlx90614, Error, Mlx9061x, DEV_ADDR};
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
    /// Read the ambient temperature
    pub fn ambient_temperature(&mut self) -> Result<f32, Error<E>> {
        let mut data = [0; 3];
        self.i2c
            .write_read(DEV_ADDR, &[mlx90614::Register::TA], &mut data)
            .map_err(Error::I2C)?;
        let pec = data[2];
        let t = u16::from(data[0]) | (u16::from(data[1]) << 8);
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }
}
