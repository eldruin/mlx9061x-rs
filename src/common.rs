use crate::{ic, register_access::{mlx90614, mlx90615}, Error, Mlx9061x, SlaveAddr};
use embedded_hal::blocking::{delay, i2c};

impl<I2C, D, IC> Mlx9061x<I2C, D, IC> {
    /// Destroy driver instance, return I²C bus and delay instance.
    pub fn destroy(self) -> (I2C, D) {
        (self.i2c, self.delay_ms)
    }
}

macro_rules! common {
    ($ic_marker:ident, $ic_reg:ident) => {
        impl<E, D, I2C> Mlx9061x<I2C, D, ic::$ic_marker>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
            D: delay::DelayMs<u8>,
        {
            /// Change the device address
            ///
            /// The address will be stored in the EEPROM.
            /// The address will be first cleared, before the address is written.
            /// After each write the configured delay will be waited.
            pub fn set_address(&mut self, address: SlaveAddr) -> Result<(), Error<E>> {
                let address = Self::get_address(address)?;
                self.write_u16($ic_reg::Register::ADDRESS, 0)?;
                self.delay_ms.delay_ms(self.eeprom_write_delay_ms);
                self.write_u16($ic_reg::Register::ADDRESS, u16::from(address))?;
                self.delay_ms.delay_ms(self.eeprom_write_delay_ms);
                Ok(())
            }
        }
    };
}
common!(Mlx90614, mlx90614);
common!(Mlx90615, mlx90615);
