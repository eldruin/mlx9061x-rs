mod common;
use crate::common::{destroy, new_mlx90614};

#[test]
fn can_create_and_destroy() {
    let sensor = new_mlx90614(&[]);
    destroy(sensor);
}
