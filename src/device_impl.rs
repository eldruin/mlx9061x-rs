use crate::Mlx9061x;
use core::marker::PhantomData;

impl<I2C, IC> Mlx9061x<I2C, IC> {
    /// Create new instance of the MLX90614 device.
    pub fn new_mlx90614(i2c: I2C) -> Self {
        Mlx9061x {
            i2c,
            _ic: PhantomData,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
