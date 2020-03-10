mod common;
use crate::common::{destroy, mlx90614, new_mlx90614, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mlx9061x::Error;

#[test]
fn can_create_and_destroy() {
    let sensor = new_mlx90614(&[]);
    destroy(sensor);
}

macro_rules! read_ambient_temp_test {
    ($name:ident, $data0:expr, $data1:expr, $data2:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut sensor = new_mlx90614(&[I2cTrans::write_read(
                ADDR,
                vec![mlx90614::Register::TA],
                vec![$data0, $data1, $data2],
            )]);
            let t = sensor.ambient_temperature().unwrap();
            assert_near!(t, $expected, 0.1);
            destroy(sensor);
        }
    };
}
read_ambient_temp_test!(can_read_ambient_temp1, 225, 57, 233, 23.19);
read_ambient_temp_test!(can_read_ambient_temp2, 97, 58, 86, 25.75);
read_ambient_temp_test!(can_read_ambient_temp3, 107, 58, 212, 25.95);
read_ambient_temp_test!(can_read_ambient_temp4, 38, 58, 102, 24.57);

#[test]
fn read_ambient_temperature_crc_mismatch() {
    let mut sensor = new_mlx90614(&[I2cTrans::write_read(
        ADDR,
        vec![mlx90614::Register::TA],
        vec![225, 57, 234],
    )]);
    assert_crc_mismatch!(sensor.ambient_temperature());
    destroy(sensor);
}
