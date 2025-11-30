use rooms::logic::*;

#[test]
fn test_and_operation() {
    let a = FuzzyBool::new(0.7);
    let b = FuzzyBool::new(0.5);
    let result = a.and(&b);
    assert!((result.value() - 0.35).abs() < 1e-10);
}

#[test]
fn test_or_operation() {
    let a = FuzzyBool::new(0.7);
    let b = FuzzyBool::new(0.5);
    let result = a.or(&b);
    assert!((result.value() - 0.85).abs() < 1e-10);
}

#[test]
fn test_not_operation() {
    let a = FuzzyBool::new(0.7);
    let result = a.not();
    assert!((result.value() - 0.3).abs() < 1e-10);
}

#[test]
fn test_clamping() {
    let a = FuzzyBool::new(1.5);
    assert!((a.value() - 1.0).abs() < 1e-10);
    let b = FuzzyBool::new(-0.5);
    assert!((b.value() - 0.0).abs() < 1e-10);
}