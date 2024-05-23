mod base;
use crate::base::{destroy, mlx90615, mlx90615::Register as Reg, new_mlx90615};
use embedded_hal_mock::eh1::{
    delay::NoopDelay,
    i2c::Transaction as I2cTrans,
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
};
use mlx9061x::{wake_mlx90615, SlaveAddr};

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
    read_object_temp_as_int,
    new_mlx90615,
    mlx90615::DEV_ADDR,
    object_temperature_as_int,
    Reg::TOBJ,
    0x26,
    0x3A,
    0xAC,
    0x18
);

read_u16_test!(
    read_ta_as_int,
    new_mlx90615,
    mlx90615::DEV_ADDR,
    ambient_temperature_as_int,
    Reg::TA,
    0x26,
    0x3A,
    0xBA,
    0x18
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

#[test]
fn can_get_id() {
    let mut sensor = new_mlx90615(&[
        I2cTrans::write_read(
            mlx90615::DEV_ADDR,
            vec![mlx90615::Register::ID0],
            vec![0x34, 0x12, 6],
        ),
        I2cTrans::write_read(
            mlx90615::DEV_ADDR,
            vec![mlx90615::Register::ID0 + 1],
            vec![0x78, 0x56, 108],
        ),
    ]);
    assert_eq!(0x1234_5678, sensor.device_id().unwrap());
    destroy(sensor);
}

#[test]
fn can_sleep() {
    let mut sensor = new_mlx90615(&[I2cTrans::write(
        mlx90615::DEV_ADDR,
        vec![mlx90615::SLEEP_COMMAND, 109],
    )]);
    sensor.sleep().unwrap();
    destroy(sensor);
}

#[test]
fn can_wake() {
    let mut scl = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let mut delay = NoopDelay::new();
    wake_mlx90615(&mut scl, &mut delay).unwrap();
    scl.done()
}
