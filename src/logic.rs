pub struct FuzzyBool {
    value: f64,
}

impl FuzzyBool {
    pub fn new(value: f64) -> Self {
        FuzzyBool {
            value: value.clamp(0., 1.),
        }
    }

    pub fn and(&self, other: &FuzzyBool) -> FuzzyBool {
        FuzzyBool::new(self.value * other.value)
    }

    pub fn or(&self, other: &FuzzyBool) -> FuzzyBool {
        FuzzyBool::new(self.value + other.value - self.value * other.value)
    }

    pub fn not(&self) -> FuzzyBool {
        FuzzyBool::new(1.0 - self.value)
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
