use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input);

    match cost {
        Ok(cost) => {
            if cost > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= cost;
                println!("You now have {} tokens.", tokens);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let prosessing_fee = 1;
    let cost_per_item = 5;
    let qty = try!(item_quantity.parse::<i32>());

    Ok(qty * cost_per_item + prosessing_fee)
}
