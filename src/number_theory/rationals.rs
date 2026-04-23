use crate::number_theory::gcd;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Rational {
    numerator: isize,
    denominator: isize,
}

mod cmp;
mod ops;

impl Rational {
    pub fn new(numerator: isize, denominator: isize) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero");
        }
        let mut rat = Rational {
            numerator,
            denominator,
        };
        rat.canonic();
        rat
    }

    pub fn canonic(&mut self) -> &mut Self {
        let gcd_value = gcd(self.numerator, self.denominator);
        self.numerator /= gcd_value;
        self.denominator /= gcd_value;
        self
    }

    pub fn inverse(&self) -> Option<Rational> {
        if self.numerator == 0 {
            return None;
        }

        let sign = if self.numerator < 0 { -1 } else { 1 };
        Some(Rational {
            numerator: sign * self.denominator,
            denominator: self.numerator.abs(),
        })
    }

    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl Default for Rational {
    fn default() -> Self {
        Self::new(0, 1)
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
