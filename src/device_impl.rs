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
}
impl<E, I2C, IC> Mlx9061x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E>,
{
    fn read_u16(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 3];
        self.i2c
            .write_read(DEV_ADDR, &[register], &mut data)
            .map_err(Error::I2C)?;
        let pec = data[2];
        Self::check_pec(
            &[
                DEV_ADDR << 1,
                register,
                (DEV_ADDR << 1) + 1,
                data[0],
                data[1],
            ],
            pec,
        )?;
        Ok(u16::from(data[0]) | (u16::from(data[1]) << 8))
    }

    fn check_pec(data: &[u8], expected: u8) -> Result<(), Error<E>> {
        if crc8(data) != expected {
            Err(Error::ChecksumMismatch)
        } else {
            Ok(())
        }
    }
}

fn crc8(data: &[u8]) -> u8 {
    // SMBus PEC: https://en.wikipedia.org/wiki/System_Management_Bus#Packet_Error_Checking
    const POLY: u8 = 7; // x^8 + x^2 + x + 1
    let mut crc = 0;
    for v in data {
        crc ^= v;
        for _ in 0..8 {
            crc = if (crc & (1 << 7)) != 0 {
                (crc << 1) ^ POLY
            } else {
                crc << 1
            };
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_crc8_table() {
        const TABLE: [u8; 256] = [
            0x00, 0x07, 0x0E, 0x09, 0x1C, 0x1B, 0x12, 0x15, 0x38, 0x3F, 0x36, 0x31, 0x24, 0x23,
            0x2A, 0x2D, 0x70, 0x77, 0x7E, 0x79, 0x6C, 0x6B, 0x62, 0x65, 0x48, 0x4F, 0x46, 0x41,
            0x54, 0x53, 0x5A, 0x5D, 0xE0, 0xE7, 0xEE, 0xE9, 0xFC, 0xFB, 0xF2, 0xF5, 0xD8, 0xDF,
            0xD6, 0xD1, 0xC4, 0xC3, 0xCA, 0xCD, 0x90, 0x97, 0x9E, 0x99, 0x8C, 0x8B, 0x82, 0x85,
            0xA8, 0xAF, 0xA6, 0xA1, 0xB4, 0xB3, 0xBA, 0xBD, 0xC7, 0xC0, 0xC9, 0xCE, 0xDB, 0xDC,
            0xD5, 0xD2, 0xFF, 0xF8, 0xF1, 0xF6, 0xE3, 0xE4, 0xED, 0xEA, 0xB7, 0xB0, 0xB9, 0xBE,
            0xAB, 0xAC, 0xA5, 0xA2, 0x8F, 0x88, 0x81, 0x86, 0x93, 0x94, 0x9D, 0x9A, 0x27, 0x20,
            0x29, 0x2E, 0x3B, 0x3C, 0x35, 0x32, 0x1F, 0x18, 0x11, 0x16, 0x03, 0x04, 0x0D, 0x0A,
            0x57, 0x50, 0x59, 0x5E, 0x4B, 0x4C, 0x45, 0x42, 0x6F, 0x68, 0x61, 0x66, 0x73, 0x74,
            0x7D, 0x7A, 0x89, 0x8E, 0x87, 0x80, 0x95, 0x92, 0x9B, 0x9C, 0xB1, 0xB6, 0xBF, 0xB8,
            0xAD, 0xAA, 0xA3, 0xA4, 0xF9, 0xFE, 0xF7, 0xF0, 0xE5, 0xE2, 0xEB, 0xEC, 0xC1, 0xC6,
            0xCF, 0xC8, 0xDD, 0xDA, 0xD3, 0xD4, 0x69, 0x6E, 0x67, 0x60, 0x75, 0x72, 0x7B, 0x7C,
            0x51, 0x56, 0x5F, 0x58, 0x4D, 0x4A, 0x43, 0x44, 0x19, 0x1E, 0x17, 0x10, 0x05, 0x02,
            0x0B, 0x0C, 0x21, 0x26, 0x2F, 0x28, 0x3D, 0x3A, 0x33, 0x34, 0x4E, 0x49, 0x40, 0x47,
            0x52, 0x55, 0x5C, 0x5B, 0x76, 0x71, 0x78, 0x7F, 0x6A, 0x6D, 0x64, 0x63, 0x3E, 0x39,
            0x30, 0x37, 0x22, 0x25, 0x2C, 0x2B, 0x06, 0x01, 0x08, 0x0F, 0x1A, 0x1D, 0x14, 0x13,
            0xAE, 0xA9, 0xA0, 0xA7, 0xB2, 0xB5, 0xBC, 0xBB, 0x96, 0x91, 0x98, 0x9F, 0x8A, 0x8D,
            0x84, 0x83, 0xDE, 0xD9, 0xD0, 0xD7, 0xC2, 0xC5, 0xCC, 0xCB, 0xE6, 0xE1, 0xE8, 0xEF,
            0xFA, 0xFD, 0xF4, 0xF3,
        ];
        for i in 0..256 {
            assert_eq!(crc8(&[i as u8]), TABLE[i]);
        }
    }

    #[test]
    fn check_crc8_array() {
        const TA: u8 = mlx90614::Register::TA;
        assert_eq!(crc8(&[0x5A << 1, TA, (0x5A << 1) + 1, 38, 58]), 102);
        assert_eq!(crc8(&[0x5A << 1, TA, (0x5A << 1) + 1, 107, 58]), 212);
        assert_eq!(crc8(&[0x5A << 1, TA, (0x5A << 1) + 1, 97, 58]), 86);
        assert_eq!(crc8(&[0x5A << 1, TA, (0x5A << 1) + 1, 225, 57]), 233);
    }
}
