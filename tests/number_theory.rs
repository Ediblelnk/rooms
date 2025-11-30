use rooms::number_theory::*;

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(101, 10), 1);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(-5,-15), -5)
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(4, 5), 20);
    assert_eq!(lcm(7, 3), 21);
    assert_eq!(lcm(0, 5), 0);
    assert_eq!(lcm(5, 0), 0);
}

#[test]
fn test_rational_addition() {
    let r1 = Rational::new(3, 2);
    let r2 = Rational::new(2, 6);
    let r3 = r1 + r2;

    assert_eq!(r3.to_string(), "11/6");
}

#[test]
fn test_rational_addition_simplify() {
    let r1 = Rational::new(3, 2);
    let r2 = Rational::new(3, 6);
    let r3 = r1 + r2;

    assert_eq!(r3.to_string(), "2/1");
}

#[test]
fn test_negative_rational_addition_simplify() {
    let r1 = Rational::new(-3, 2);
    let r2 = Rational::new(3, -6);
    let r3 = r1 + r2;

    assert_eq!(r3.to_string(), "-2/1");
}