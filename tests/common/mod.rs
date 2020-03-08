use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use mlx9061x::{ic, Mlx9061x};
pub const ADDR: u8 = 0x5A;

pub mod mlx90614 {
    pub struct Register {}
    impl Register {
        pub const TA: u8 = 0x06;
    }
}

pub fn new_mlx90614(transactions: &[I2cTrans]) -> Mlx9061x<I2cMock, ic::Mlx90614> {
    Mlx9061x::new_mlx90614(I2cMock::new(transactions))
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
