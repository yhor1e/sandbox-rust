use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let prosessing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    match qty {
        Ok(qty) => Ok(qty * cost_per_item + prosessing_fee),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171))
    }

    #[test]
    fn item_quantity_is_a_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        )
    }

}
