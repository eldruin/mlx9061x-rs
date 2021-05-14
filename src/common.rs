use crate::{
    ic,
    register_access::{mlx90614, mlx90615},
    Error, Mlx9061x, SlaveAddr,
};
use embedded_hal::blocking::{delay::DelayMs, i2c};

impl<I2C, IC> Mlx9061x<I2C, IC> {
    /// Destroy driver instance, return I²C bus.
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
            /// After each write the configured delay will be waited except the last time.
            pub fn set_address<D: DelayMs<u8>>(
                &mut self,
                address: SlaveAddr,
                delay_ms: &mut D,
            ) -> Result<(), Error<E>> {
                let address = Self::get_address(address, $ic_reg::DEV_ADDR)?;
                self.write_u16_eeprom($ic_reg::Register::ADDRESS, u16::from(address), delay_ms)?;
                self.address = address;
                Ok(())
            }

            /// Enter sleep mode
            ///
            /// After entering sleep, either destroy this driver to get the SDA/SCL pins back
            /// and call the `wake()` method or perform a hardware POR to wake the device.
            pub fn sleep(&mut self) -> Result<(), Error<E>> {
                self.write_u8($ic_reg::SLEEP_COMMAND)
            }
        }
    };
}
common!(Mlx90614, mlx90614);
common!(Mlx90615, mlx90615);
