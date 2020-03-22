use linux_embedded_hal::I2cdev;
use mlx9061x::{Mlx9061x, SlaveAddr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Mlx9061x::new_mlx90614(dev, SlaveAddr::default(), 5).unwrap();
    loop {
        let obj_temp = sensor.object1_temperature().unwrap();
        println!("Object temperature: {:.2}ºC", obj_temp);
    }
}
