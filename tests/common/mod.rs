use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use mlx9061x::{ic, Mlx9061x};

pub fn new_mlx90614(transactions: &[I2cTrans]) -> Mlx9061x<I2cMock, ic::Mlx90614> {
    Mlx9061x::new_mlx90614(I2cMock::new(transactions))
}

pub fn destroy<IC>(sensor: Mlx9061x<I2cMock, IC>) {
    sensor.destroy().done();
}
