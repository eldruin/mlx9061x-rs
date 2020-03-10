pub const DEV_ADDR: u8 = 0x5A;

pub mod mlx90614 {
    pub struct Register {}
    impl Register {
        pub const TA: u8 = 0x06;
        pub const TOBJ1: u8 = 0x07;
        pub const TOBJ2: u8 = 0x08;
    }
}
