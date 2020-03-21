mod common;
use crate::common::{destroy, mlx90615, mlx90615::Register as Reg, new_mlx90615};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mlx9061x::Error;

#[test]
fn can_create_and_destroy() {
    let sensor = new_mlx90615(&[]);
    destroy(sensor);
}

macro_rules! read_f32_test {
    ($name:ident, $method:ident, $reg:expr, $data0:expr, $data1:expr, $data2:expr, $expected:expr) => {
        read_f32_test_base!(
            $name,
            new_mlx90615,
            mlx90615::DEV_ADDR,
            $method,
            $reg,
            $data0,
            $data1,
            $data2,
            $expected
        );
    };
}
read_f32_test!(read_ta1, ambient_temperature, Reg::TA, 225, 57, 251, 23.19);
read_f32_test!(read_ta2, ambient_temperature, Reg::TA, 97, 58, 68, 25.75);
read_f32_test!(read_ta3, ambient_temperature, Reg::TA, 107, 58, 198, 25.95);
read_f32_test!(read_ta4, ambient_temperature, Reg::TA, 38, 58, 116, 24.57);

read_f32_test!(
    read_object_temp,
    object_temperature,
    Reg::TOBJ,
    38,
    58,
    98,
    24.57
);

#[test]
fn read_ambient_temperature_crc_mismatch() {
    let mut sensor = new_mlx90615(&[I2cTrans::write_read(
        mlx90615::DEV_ADDR,
        vec![Reg::TA],
        vec![225, 57, 234],
    )]);
    assert_crc_mismatch!(sensor.ambient_temperature());
    destroy(sensor);
}

read_u16_test!(
    read_raw_ir,
    new_mlx90615,
    mlx90615::DEV_ADDR,
    raw_ir,
    Reg::RAW_IR,
    0x26,
    0x3A,
    0x4E,
    0x3A26
);
