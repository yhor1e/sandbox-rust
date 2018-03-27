pub fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]

mod tests {
    use super::times_two;

    #[test]
    fn retruns_twice_of_positive_numbers() {
        assert_eq!(4, times_two(2));
    }

    #[test]
    fn retruns_twice_of_negative_numbers() {
        assert_eq!(-4, times_two(-2));
    }
}
