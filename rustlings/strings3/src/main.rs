fn string_slice(arg: &str) {
    println!("{}", arg);
}

fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("h1"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  Hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("my sHiFt key is sTiCkY".to_lowercase());
}
