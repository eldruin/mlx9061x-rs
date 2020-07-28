use crate::{
    ic,
    register_access::{mlx90614, mlx90615},
    Error, Mlx9061x, SlaveAddr,
};
use embedded_hal::blocking::{delay, i2c};

impl<I2C, IC> Mlx9061x<I2C, IC> {
    /// Destroy driver instance, return I²C bus and delay instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

macro_rules! common {
    ($ic_marker:ident, $ic_reg:ident) => {
        impl<E, I2C> Mlx9061x<I2C, ic::$ic_marker>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Change the device address
            ///
            /// The address will be stored in the EEPROM.
            /// The address will be first cleared, before the new one is written.
            /// After each write the configured delay will be waited.
            pub fn set_address<D: delay::DelayMs<u8>>(
                &mut self,
                address: SlaveAddr,
                delay_ms: &mut D,
            ) -> Result<(), Error<E>> {
                let address = Self::get_address(address, $ic_reg::DEV_ADDR)?;
                self.write_u16_eeprom($ic_reg::Register::ADDRESS, u16::from(address), delay_ms)?;
                self.address = address;
                Ok(())
            }
        }
    };
}
common!(Mlx90614, mlx90614);
common!(Mlx90615, mlx90615);
