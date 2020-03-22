use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use mlx9061x::{ic, Mlx9061x, SlaveAddr};

#[allow(unused)]
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
    }
}

#[allow(unused)]
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
    }
}

#[allow(unused)]
pub fn new_mlx90614(transactions: &[I2cTrans]) -> Mlx9061x<I2cMock, ic::Mlx90614> {
    Mlx9061x::new_mlx90614(I2cMock::new(transactions), SlaveAddr::default(), 5).unwrap()
}

#[allow(unused)]
pub fn new_mlx90615(transactions: &[I2cTrans]) -> Mlx9061x<I2cMock, ic::Mlx90615> {
    Mlx9061x::new_mlx90615(I2cMock::new(transactions), SlaveAddr::default(), 5).unwrap()
}

pub fn destroy<IC>(sensor: Mlx9061x<I2cMock, IC>) {
    sensor.destroy().done();
}

#[macro_export]
macro_rules! assert_near {
    ($value:expr, $expected:expr, $epsilon:expr) => {
        assert!(($value - $epsilon) < $expected);
        assert!(($value + $epsilon) > $expected);
    };
}

#[macro_export]
macro_rules! assert_crc_mismatch {
    ($result: expr) => {
        assert_error!($result, ChecksumMismatch);
    };
}

#[macro_export]
macro_rules! assert_error {
    ($result: expr, $error:ident) => {
        match $result {
            Err(Error::$error) => (),
            _ => panic!("Should have returned error."),
        }
    };
}

#[macro_export]
macro_rules! read_f32_test_base {
    ($name:ident, $create:ident, $address:expr, $method:ident, $reg:expr, $data0:expr, $data1:expr, $data2:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut sensor = $create(&[I2cTrans::write_read(
                $address,
                vec![$reg],
                vec![$data0, $data1, $data2],
            )]);
            let t = sensor.$method().unwrap();
            assert_near!(t, $expected, 0.1);
            destroy(sensor);
        }
    };
}

#[macro_export]
macro_rules! read_u16_test {
    ($name:ident, $create:ident, $address:expr, $method:ident, $reg:expr, $data0:expr, $data1:expr, $data2:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut sensor = $create(&[I2cTrans::write_read(
                $address,
                vec![$reg],
                vec![$data0, $data1, $data2],
            )]);
            let t = sensor.$method().unwrap();
            assert_eq!(t, $expected);
            destroy(sensor);
        }
    };
}
