fn main() {
    my_macro!();
}

#[use_macro]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }
}
