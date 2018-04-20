fn main() {
    let string = format!("my friend");
    greet(&string);
    greet(&string);
}

fn greet(name: &String) {
    name.replace("my ", "");

    println!("Hello, {}", name.split_at(3).1);
}
