use crate::{Error, Mlx9061x, SlaveAddr};
use embedded_hal::{delay::DelayNs, i2c::I2c};
use smbus_pec::pec;

fn msb_lsb_to_sign_magnitude(value: u16) -> i16 {
    let sign_bit = value & 0b1000_0000_0000_0000;

    let value = (value & 0b0111_1111_1111_1111) as i16;

    if sign_bit != 0 {
        -value
    } else {
        value
    }
}

pub mod mlx90614 {
    const EEPROM_COMMAND: u8 = 0x20;
    pub const SLEEP_COMMAND: u8 = 0xFF;
    pub const WAKE_DELAY_MS: u8 = 33;
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
    pub const SLEEP_COMMAND: u8 = 0xC6;
    pub const WAKE_DELAY_MS: u8 = 39;
    pub const DEV_ADDR: u8 = 0x5B;

    pub struct Register {}

    impl Register {
        pub const RAW_IR: u8 = 0x05 | RAM_COMMAND;
        pub const TA: u8 = 0x06 | RAM_COMMAND;
        pub const TOBJ: u8 = 0x07 | RAM_COMMAND;
        pub const ADDRESS: u8 = /*0x00 |*/ EEPROM_COMMAND;
        pub const EMISSIVITY: u8 = 0x03 | EEPROM_COMMAND;
        pub const ID0: u8 = 0x0E | EEPROM_COMMAND;
    }
}

impl<E, I2C, IC> Mlx9061x<I2C, IC>
where
    I2C: I2c<Error = E>,
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

    pub(crate) fn read_i16(&mut self, register: u8) -> Result<i16, Error<E>> {
        let value = self.read_u16(register)?;
        Ok(msb_lsb_to_sign_magnitude(value))
    }

    pub(crate) fn write_u8(&mut self, command: u8) -> Result<(), Error<E>> {
        let pec = pec(&[self.address << 1, command]);
        self.i2c
            .write(self.address, &[command, pec])
            .map_err(Error::I2C)
    }

    pub(crate) fn write_u16(&mut self, command: u8, data: u16) -> Result<(), Error<E>> {
        let low = data as u8;
        let high = (data >> 8) as u8;
        let pec = pec(&[self.address << 1, command, low, high]);
        self.i2c
            .write(self.address, &[command, low, high, pec])
            .map_err(Error::I2C)
    }

    pub(crate) fn write_u16_eeprom<D: DelayNs>(
        &mut self,
        command: u8,
        data: u16,
        delay: &mut D,
    ) -> Result<(), Error<E>> {
        self.write_u16(command, 0)?;
        delay.delay_ms(u32::from(self.eeprom_write_delay_ms));
        self.write_u16(command, data)
    }

    pub(crate) fn check_pec(data: &[u8], expected: u8) -> Result<(), Error<E>> {
        if pec(data) != expected {
            Err(Error::ChecksumMismatch)
        } else {
            Ok(())
        }
    }

    pub(crate) fn get_address(address: SlaveAddr, default: u8) -> Result<u8, Error<E>> {
        match address {
            SlaveAddr::Default => Ok(default),
            SlaveAddr::Alternative(0) => Err(Error::InvalidInputData),
            SlaveAddr::Alternative(a) if a > 127 => Err(Error::InvalidInputData),
            SlaveAddr::Alternative(a) => Ok(a),
        }
    }
}

#[cfg(test)]
mod msb_lsb_to_sign_magnitude_tests {
    use crate::register_access::msb_lsb_to_sign_magnitude;

    /// macro to define test cases
    #[macro_export]
    macro_rules! msb_lsb_to_sign_magnitude_test {
        ($name:ident, $msb:expr, $lsb:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let value = $msb << 8 | $lsb;

                let result = msb_lsb_to_sign_magnitude(value);

                assert_eq!(
                    result, $expected,
                    "For MSB: {:#X}, LSB: {:#X}, expected: {}, got: {}",
                    $msb, $lsb, $expected, result
                );
            }
        };
    }

    msb_lsb_to_sign_magnitude_test!(test_msb_lsb_zero, 0x00, 0x00, 0);

    msb_lsb_to_sign_magnitude_test!(test_msb_lsb_positive_258, 0x01, 0x02, 258);

    msb_lsb_to_sign_magnitude_test!(test_msb_lsb_max_positive, 0x7F, 0xFF, 32767);

    msb_lsb_to_sign_magnitude_test!(test_msb_lsb_min_negative, 0x80, 0x00, -0);

    msb_lsb_to_sign_magnitude_test!(test_msb_lsb_negative_258, 0x81, 0x02, -258);

    msb_lsb_to_sign_magnitude_test!(test_msb_lsb_negative_32767, 0xFF, 0xFF, -32767);
    
    msb_lsb_to_sign_magnitude_test!(test_msb_lsb_negative_one, 0x80, 0x01, -1);
}
