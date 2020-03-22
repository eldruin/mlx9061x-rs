use crate::{crc8, ic, Error, Mlx9061x, SlaveAddr};
use embedded_hal::blocking::{delay, i2c};

pub mod mlx90614 {
    const EEPROM_COMMAND: u8 = 0x20;
    pub const DEV_ADDR: u8 = 0x5A;
    pub struct Register {}
    impl Register {
        pub const RAW_IR1: u8 = 0x04;
        pub const RAW_IR2: u8 = 0x05;
        pub const TA: u8 = 0x06;
        pub const TOBJ1: u8 = 0x07;
        pub const TOBJ2: u8 = 0x08;
        pub const EMISSIVITY: u8 = 0x04 | EEPROM_COMMAND;
        pub const ADDRESS: u8 = 0x0E | EEPROM_COMMAND;
        pub const ID0: u8 = 0x1C | EEPROM_COMMAND;
    }
}

pub mod mlx90615 {
    const EEPROM_COMMAND: u8 = 0x10;
    const RAM_COMMAND: u8 = 0x20;
    pub const DEV_ADDR: u8 = 0x5B;
    pub struct Register {}
    impl Register {
        pub const RAW_IR: u8 = 0x05 | RAM_COMMAND;
        pub const TA: u8 = 0x06 | RAM_COMMAND;
        pub const TOBJ: u8 = 0x07 | RAM_COMMAND;
        pub const ADDRESS: u8 = 0x00 | EEPROM_COMMAND;
        pub const EMISSIVITY: u8 = 0x03 | EEPROM_COMMAND;
        pub const ID0: u8 = 0x0E | EEPROM_COMMAND;
    }
}

macro_rules! reg_access {
    ($ic_marker:ident, $ic_reg:ident) => {
        impl<E, I2C> Mlx9061x<I2C, ic::$ic_marker>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            pub(crate) fn read_u16(&mut self, register: u8) -> Result<u16, Error<E>> {
                let mut data = [0; 3];
                self.i2c
                    .write_read(self.address, &[register], &mut data)
                    .map_err(Error::I2C)?;
                let pec = data[2];
                Self::check_pec(
                    &[
                        self.address << 1,
                        register,
                        (self.address << 1) + 1,
                        data[0],
                        data[1],
                    ],
                    pec,
                )?;
                Ok(u16::from(data[0]) | (u16::from(data[1]) << 8))
            }

            pub(crate) fn write_u16(&mut self, command: u8, data: u16) -> Result<(), Error<E>> {
                let low = data as u8;
                let high = (data >> 8) as u8;
                let pec = crc8(&[self.address << 1, command, low, high]);
                self.i2c
                    .write(self.address, &[command, low, high, pec])
                    .map_err(Error::I2C)
            }

            pub(crate) fn write_u16_eeprom<D: delay::DelayMs<u8>>(
                &mut self,
                command: u8,
                data: u16,
                delay: &mut D,
            ) -> Result<(), Error<E>> {
                self.write_u16(command, 0)?;
                delay.delay_ms(self.eeprom_write_delay_ms);
                self.write_u16(command, data)?;
                delay.delay_ms(self.eeprom_write_delay_ms);
                Ok(())
            }

            pub(crate) fn get_address(address: SlaveAddr) -> Result<u8, Error<E>> {
                match address {
                    SlaveAddr::Default => Ok($ic_reg::DEV_ADDR),
                    SlaveAddr::Alternative(a) if a == 0 => return Err(Error::InvalidInputData),
                    SlaveAddr::Alternative(a) if a > 127 => return Err(Error::InvalidInputData),
                    SlaveAddr::Alternative(a) => Ok(a),
                }
            }
        }
    };
}
reg_access!(Mlx90614, mlx90614);
reg_access!(Mlx90615, mlx90615);

impl<E, I2C, IC> Mlx9061x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub(crate) fn check_pec(data: &[u8], expected: u8) -> Result<(), Error<E>> {
        if crc8(data) != expected {
            Err(Error::ChecksumMismatch)
        } else {
            Ok(())
        }
    }
}
