use super::Rational;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}
