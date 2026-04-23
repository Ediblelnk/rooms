use crate::properties::Q;

pub trait Identity {
    fn identity() -> Self;
}

impl Identity for i32 {
    fn identity() -> Self {
        1
    }
}

impl Identity for f64 {
    fn identity() -> Self {
        1.0
    }
}

impl Identity for Q {
    fn identity() -> Self {
        Q::new(1, 1)
    }
}
