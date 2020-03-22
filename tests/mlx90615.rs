mod base;
use crate::base::{destroy, mlx90615, mlx90615::Register as Reg, new_mlx90615};
use embedded_hal_mock::{delay::MockNoop as NoopDelay, i2c::Transaction as I2cTrans};
use mlx9061x::SlaveAddr;

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

#[test]
fn can_change_address() {
    let mut sensor = new_mlx90615(&[
        I2cTrans::write(mlx90615::DEV_ADDR, vec![Reg::ADDRESS, 0, 0, 78]),
        I2cTrans::write(mlx90615::DEV_ADDR, vec![Reg::ADDRESS, 0x5C, 0, 190]),
    ]);
    sensor
        .set_address(SlaveAddr::Alternative(0x5C), &mut NoopDelay {})
        .unwrap();
    destroy(sensor);
}

#[test]
fn can_set_emissivity() {
    let mut sensor = new_mlx90615(&[
        I2cTrans::write(mlx90615::DEV_ADDR, vec![Reg::EMISSIVITY, 0, 0, 243]),
        I2cTrans::write(mlx90615::DEV_ADDR, vec![Reg::EMISSIVITY, 205, 44, 51]),
    ]);
    sensor.set_emissivity(0.7, &mut NoopDelay {}).unwrap();
    destroy(sensor);
}

read_f32_test!(read_emiss, emissivity, Reg::EMISSIVITY, 205, 44, 235, 0.7);
