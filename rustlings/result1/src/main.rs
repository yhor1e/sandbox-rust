#[derive(PartialEq, Debug)]
struct PositiveNonzeroInterger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInterger {
    fn new(value: i64) -> Result<PositiveNonzeroInterger, CreationError> {
        println!("{}", value);
        if value > 0 {
            println!("1");
            Ok(PositiveNonzeroInterger(value as u64))
        } else if value == 0 {
            println!("2");
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInterger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInterger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInterger::new(0));
}
