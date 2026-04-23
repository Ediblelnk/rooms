use super::Rational;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        let mut ret = Rational::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        );
        ret.canonic();
        ret
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        };
        self.canonic();
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational::new(-self.numerator, self.denominator)
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        self + (-other)
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        let mut ret = Rational::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        );
        ret.canonic();
        ret
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        };
        self.canonic();
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {
        self * other.inverse().expect("Division by zero rational")
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inverse().expect("Division by zero rational");
    }
}
