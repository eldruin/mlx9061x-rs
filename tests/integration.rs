mod common;
use crate::common::{destroy, mlx90614::Register as Reg4, new_mlx90614, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mlx9061x::Error;

#[test]
fn can_create_and_destroy() {
    let sensor = new_mlx90614(&[]);
    destroy(sensor);
}

macro_rules! read_f32_test {
    ($name:ident, $method:ident, $reg:expr, $data0:expr, $data1:expr, $data2:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut sensor = new_mlx90614(&[I2cTrans::write_read(
                ADDR,
                vec![$reg],
                vec![$data0, $data1, $data2],
            )]);
            let t = sensor.$method().unwrap();
            assert_near!(t, $expected, 0.1);
            destroy(sensor);
        }
    };
}
read_f32_test!(read_ta1, ambient_temperature, Reg4::TA, 225, 57, 233, 23.19);
read_f32_test!(read_ta2, ambient_temperature, Reg4::TA, 97, 58, 86, 25.75);
read_f32_test!(read_ta3, ambient_temperature, Reg4::TA, 107, 58, 212, 25.95);
read_f32_test!(read_ta4, ambient_temperature, Reg4::TA, 38, 58, 102, 24.57);

read_f32_test!(
    read_object1_temp,
    object1_temperature,
    Reg4::TOBJ1,
    38,
    58,
    112,
    24.57
);

read_f32_test!(
    read_object2_temp,
    object2_temperature,
    Reg4::TOBJ2,
    38,
    58,
    162,
    24.57
);

#[test]
fn read_ambient_temperature_crc_mismatch() {
    let mut sensor = new_mlx90614(&[I2cTrans::write_read(
        ADDR,
        vec![Reg4::TA],
        vec![225, 57, 234],
    )]);
    assert_crc_mismatch!(sensor.ambient_temperature());
    destroy(sensor);
}

macro_rules! read_u16_test {
    ($name:ident, $method:ident, $reg:expr, $data0:expr, $data1:expr, $data2:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut sensor = new_mlx90614(&[I2cTrans::write_read(
                ADDR,
                vec![$reg],
                vec![$data0, $data1, $data2],
            )]);
            let v = sensor.$method().unwrap();
            assert_eq!(v, $expected);
            destroy(sensor);
        }
    };
}

read_u16_test!(
    read_raw_ir1,
    raw_ir_channel1,
    Reg4::RAW_IR1,
    0x26,
    0x3A,
    0x4A,
    0x3A26
);

read_u16_test!(
    read_raw_ir2,
    raw_ir_channel2,
    Reg4::RAW_IR2,
    0x26,
    0x3A,
    0x5C,
    0x3A26
);
