#[cfg(target_os = "linux")]
use linux_embedded_hal::I2cdev;
#[cfg(target_os = "linux")]
use mlx9061x::{Mlx9061x, SlaveAddr};

#[cfg(target_os = "linux")]
fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let addr = SlaveAddr::default();
    let mut sensor = Mlx9061x::new_mlx90614(dev, addr, 5).unwrap();
    loop {
        let obj_temp = sensor.object1_temperature().unwrap();
        println!("Object temperature: {:.2}ºC", obj_temp);
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    panic!("This example can only be run on Linux.");
}
