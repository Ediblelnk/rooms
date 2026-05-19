pub mod rationals;
pub use rationals::*;

pub mod primes;

pub fn gcd(a: isize, b: isize) -> isize {
    let mut temp;
    let mut a = a;
    let mut b = b;
    while b != 0 {
        temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(a: isize, b: isize) -> isize {
    (a * b) / gcd(a, b)
}

impl From<isize> for Rational {
    fn from(value: isize) -> Self {
        Self::new(value, 1)
    }
}
