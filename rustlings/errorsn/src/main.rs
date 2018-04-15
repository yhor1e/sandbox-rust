use std::error;
use std::fmt;
use std::io;

fn read_and_validate(b: &mut io::BufRead) -> Result<PositiveNonzeroInterger, ???> {

    let mut line = String::new();
    b.read_line(&mut line);
    let num: i64 = line.trim().parse();
    let answer = PositiveNonzeroInterger::new(num);
    answer
}

fn test_with_str(s: &str) -> Result<PositiveNonzeroInterger, Box<error: Error>> {
    let mut b = io::BufReader::new(s.as_bytes());
    read_and_validate(&mut b)
}

#[test]
fn test_success() {
    let x = test_with_str("42/n");
    assert_eq!(PositiveNonzeroInterger(42),x.unwrap());
}

#[test]
fn test_not_num() {
    let x = test_with_str("eleven billion/n");
    assert_eq(x.is_err());
}
