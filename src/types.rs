/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// IC marker
pub mod ic {
    /// MLX90614 IC marker
    pub struct Mlx90614(());
}
