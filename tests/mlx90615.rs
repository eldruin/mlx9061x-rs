mod common;
use crate::common::{destroy, mlx90615, mlx90615::Register as Reg, new_mlx90615};
use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use mlx9061x::{Error, Mlx9061x, SlaveAddr};

#[test]
fn can_create_and_destroy() {
    let sensor = new_mlx90615(&[]);
    destroy(sensor);
}

#[test]
fn wrong_address_raises_error() {
    assert_error!(
        Mlx9061x::new_mlx90615(I2cMock::new(&[]), SlaveAddr::Alternative(0), 5),
        InvalidInputData
    );
    assert_error!(
        Mlx9061x::new_mlx90615(I2cMock::new(&[]), SlaveAddr::Alternative(128), 5),
        InvalidInputData
    );
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
read_f32_test!(read_ta1, ambient_temperature, Reg::TA, 225, 57, 53, 23.19);
read_f32_test!(read_ta2, ambient_temperature, Reg::TA, 97, 58, 138, 25.75);
read_f32_test!(read_ta3, ambient_temperature, Reg::TA, 107, 58, 8, 25.95);
read_f32_test!(read_ta4, ambient_temperature, Reg::TA, 38, 58, 186, 24.57);

read_f32_test!(
    read_object_temp,
    object_temperature,
    Reg::TOBJ,
    38,
    58,
    172,
    24.57
);

#[test]
fn read_ambient_temperature_crc_mismatch() {
    let mut sensor = new_mlx90615(&[I2cTrans::write_read(
        mlx90615::DEV_ADDR,
        vec![Reg::TA],
        vec![225, 57, 54],
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
    0x80,
    0x3A26
);
