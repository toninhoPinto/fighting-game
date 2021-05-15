pub(crate) trait Sign {
    fn sign(&self) -> Self;
}

impl Sign for f64 {
    fn sign(&self) -> Self {
        if *self == 0f64 {
            0f64
        } else {
            self.signum()
        }
    }
}

impl Sign for i8 {
    fn sign(&self) -> Self {
        if *self == 0i8 {
            0i8
        } else {
            self.signum()
        }
    }
}

impl Sign for i32 {
    fn sign(&self) -> Self {
        if *self == 0i32 {
            0i32
        } else {
            self.signum()
        }
    }
}