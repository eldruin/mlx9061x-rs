mod base;
use crate::base::{destroy, mlx90614, mlx90615, new_mlx90614, new_mlx90615};
use embedded_hal_mock::eh1::{
    delay::NoopDelay,
    i2c::{Mock as I2cMock, Transaction as I2cTrans},
};
use mlx9061x::{Error, Mlx9061x, SlaveAddr};

macro_rules! tests {
    ($create:ident, $ic:ident) => {
        #[test]
        fn can_create_and_destroy() {
            let sensor = $create(&[]);
            destroy(sensor);
        }

        #[test]
        fn wrong_address_raises_error() {
            assert_error!(
                Mlx9061x::$create(I2cMock::new(&[]), SlaveAddr::Alternative(0), 5),
                InvalidInputData
            );
            assert_error!(
                Mlx9061x::$create(I2cMock::new(&[]), SlaveAddr::Alternative(128), 5),
                InvalidInputData
            );
        }

        #[test]
        fn read_ambient_temperature_crc_mismatch() {
            let mut sensor = $create(&[I2cTrans::write_read(
                $ic::DEV_ADDR,
                vec![$ic::Register::TA],
                vec![225, 57, 234],
            )]);
            assert_crc_mismatch!(sensor.ambient_temperature());
            destroy(sensor);
        }

        #[test]
        fn set_wrong_address_returns_error() {
            let mut sensor = $create(&[]);
            assert_error!(
                sensor.set_address(SlaveAddr::Alternative(0), &mut NoopDelay {}),
                InvalidInputData
            );
            destroy(sensor);
        }

        #[test]
        fn set_wrong_too_small_emissivity_returns_error() {
            let mut sensor = $create(&[]);
            assert_error!(
                sensor.set_emissivity(-0.1, &mut NoopDelay {}),
                InvalidInputData
            );
            destroy(sensor);
        }

        #[test]
        fn set_wrong_too_big_emissivity_returns_error() {
            let mut sensor = $create(&[]);
            assert_error!(
                sensor.set_emissivity(1.1, &mut NoopDelay {}),
                InvalidInputData
            );
            destroy(sensor);
        }
    };
}

mod mlx90614_tests {
    use super::*;
    tests!(new_mlx90614, mlx90614);
}

mod mlx90615_tests {
    use super::*;
    tests!(new_mlx90615, mlx90615);
}
