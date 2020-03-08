mod common;
use crate::common::{destroy, mlx90614, new_mlx90614, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

#[test]
fn can_create_and_destroy() {
    let sensor = new_mlx90614(&[]);
    destroy(sensor);
}

#[test]
fn can_read_ambient_temperature() {
    let mut sensor = new_mlx90614(&[I2cTrans::write_read(
        ADDR,
        vec![mlx90614::Register::TA],
        vec![225, 57, 233],
    )]);
    let t = sensor.ambient_temperature().unwrap();
    assert_near!(t, 23.19, 0.1);
    destroy(sensor);
}

#[test]
fn can_read_ambient_temperature2() {
    let mut sensor = new_mlx90614(&[I2cTrans::write_read(
        ADDR,
        vec![mlx90614::Register::TA],
        vec![97, 58, 86],
    )]);
    let t = sensor.ambient_temperature().unwrap();
    assert_near!(t, 25.75, 0.1);
    destroy(sensor);
}

#[test]
fn can_read_ambient_temperature3() {
    let mut sensor = new_mlx90614(&[I2cTrans::write_read(
        ADDR,
        vec![mlx90614::Register::TA],
        vec![107, 58, 212],
    )]);
    let t = sensor.ambient_temperature().unwrap();
    assert_near!(t, 25.95, 0.1);
    destroy(sensor);
}

#[test]
fn can_read_ambient_temperature4() {
    let mut sensor = new_mlx90614(&[I2cTrans::write_read(
        ADDR,
        vec![mlx90614::Register::TA],
        vec![38, 58, 102],
    )]);
    let t = sensor.ambient_temperature().unwrap();
    assert_near!(t, 24.57, 0.1);
    destroy(sensor);
}
