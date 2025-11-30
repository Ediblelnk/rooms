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

pub struct Rational {
    numerator: isize,
    denominator: isize,
}

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

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl std::ops::Add for Rational {
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

impl std::ops::AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        };
        self.canonic();
    }
}

impl std::ops::Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational::new(-self.numerator, self.denominator)
    }
}

impl std::ops::Sub for Rational {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        self + (-other)
    }
}

impl std::ops::SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl std::ops::Mul for Rational {
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

impl std::ops::MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        };
        self.canonic();
    }
}

impl std::ops::Div for Rational {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {
        self * other.inverse().expect("Division by zero rational")
    }
}

impl std::ops::DivAssign for Rational {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inverse().expect("Division by zero rational");
    }
}

impl std::cmp::PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
    }
}

impl std::cmp::Eq for Rational {}

impl std::cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}