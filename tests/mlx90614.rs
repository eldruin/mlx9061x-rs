mod base;
use crate::base::{destroy, mlx90614, mlx90614::Register as Reg, new_mlx90614};
use embedded_hal_mock::eh1::{
    delay::NoopDelay,
    i2c::Transaction as I2cTrans,
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
};
use mlx9061x::{wake_mlx90614, SlaveAddr};

macro_rules! read_f32_test {
    ($name:ident, $method:ident, $reg:expr, $data0:expr, $data1:expr, $data2:expr, $expected:expr) => {
        read_f32_test_base!(
            $name,
            new_mlx90614,
            mlx90614::DEV_ADDR,
            $method,
            $reg,
            $data0,
            $data1,
            $data2,
            $expected
        );
    };
}
read_f32_test!(read_ta1, ambient_temperature, Reg::TA, 225, 57, 233, 23.19);
read_f32_test!(read_ta2, ambient_temperature, Reg::TA, 97, 58, 86, 25.75);
read_f32_test!(read_ta3, ambient_temperature, Reg::TA, 107, 58, 212, 25.95);
read_f32_test!(read_ta4, ambient_temperature, Reg::TA, 38, 58, 102, 24.57);

read_f32_test!(
    read_object1_temp,
    object1_temperature,
    Reg::TOBJ1,
    38,
    58,
    112,
    24.57
);

read_f32_test!(
    read_object2_temp,
    object2_temperature,
    Reg::TOBJ2,
    38,
    58,
    162,
    24.57
);

read_u16_test!(
    read_ta_as_int,
    new_mlx90614,
    mlx90614::DEV_ADDR,
    ambient_temperature_as_int,
    Reg::TA,
    0x0,
    0x3A,
    0xB6,
    0x17
);

read_u16_test!(
    read_object1_temp_as_int,
    new_mlx90614,
    mlx90614::DEV_ADDR,
    object1_temperature_as_int,
    Reg::TOBJ1,
    0x26,
    0x3A,
    0x70,
    0x18
);

read_u16_test!(
    read_object2_temp_as_int,
    new_mlx90614,
    mlx90614::DEV_ADDR,
    object2_temperature_as_int,
    Reg::TOBJ2,
    0x26,
    0x3A,
    0xA2,
    0x18
);

read_u16_test!(
    read_raw_ir1,
    new_mlx90614,
    mlx90614::DEV_ADDR,
    raw_ir_channel1,
    Reg::RAW_IR1,
    0x26,
    0x3A,
    0x4A,
    0x3A26
);

read_u16_test!(
    read_raw_ir2,
    new_mlx90614,
    mlx90614::DEV_ADDR,
    raw_ir_channel2,
    Reg::RAW_IR2,
    0x26,
    0x3A,
    0x5C,
    0x3A26
);

#[test]
fn can_change_address() {
    let mut sensor = new_mlx90614(&[
        I2cTrans::write(mlx90614::DEV_ADDR, vec![Reg::ADDRESS, 0, 0, 175]),
        I2cTrans::write(mlx90614::DEV_ADDR, vec![Reg::ADDRESS, 0x5C, 0, 95]),
    ]);
    sensor
        .set_address(SlaveAddr::Alternative(0x5C), &mut NoopDelay {})
        .unwrap();
    destroy(sensor);
}

#[test]
fn can_set_emissivity() {
    let mut sensor = new_mlx90614(&[
        I2cTrans::write(mlx90614::DEV_ADDR, vec![Reg::EMISSIVITY, 0, 0, 40]),
        I2cTrans::write(mlx90614::DEV_ADDR, vec![Reg::EMISSIVITY, 51, 179, 254]),
    ]);
    sensor.set_emissivity(0.7, &mut NoopDelay {}).unwrap();
    destroy(sensor);
}

read_f32_test!(read_emiss, emissivity, Reg::EMISSIVITY, 51, 179, 36, 0.7);

#[test]
fn can_get_id() {
    let mut sensor = new_mlx90614(&[
        I2cTrans::write_read(
            mlx90614::DEV_ADDR,
            vec![mlx90614::Register::ID0],
            vec![0x34, 0x12, 246],
        ),
        I2cTrans::write_read(
            mlx90614::DEV_ADDR,
            vec![mlx90614::Register::ID0 + 1],
            vec![0x78, 0x56, 156],
        ),
        I2cTrans::write_read(
            mlx90614::DEV_ADDR,
            vec![mlx90614::Register::ID0 + 2],
            vec![0xBC, 0x9A, 117],
        ),
        I2cTrans::write_read(
            mlx90614::DEV_ADDR,
            vec![mlx90614::Register::ID0 + 3],
            vec![0xF0, 0xDE, 31],
        ),
    ]);
    assert_eq!(0x1234_5678_9ABC_DEF0, sensor.device_id().unwrap());
    destroy(sensor);
}

#[test]
fn can_sleep() {
    let mut sensor = new_mlx90614(&[I2cTrans::write(
        mlx90614::DEV_ADDR,
        vec![mlx90614::SLEEP_COMMAND, 232],
    )]);
    sensor.sleep().unwrap();
    destroy(sensor);
}

#[test]
fn can_wake() {
    let mut scl = PinMock::new(&[PinTrans::set(PinState::High)]);
    let mut sda = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let mut delay = NoopDelay::new();
    wake_mlx90614(&mut scl, &mut sda, &mut delay).unwrap();
    scl.done();
    sda.done()
}
