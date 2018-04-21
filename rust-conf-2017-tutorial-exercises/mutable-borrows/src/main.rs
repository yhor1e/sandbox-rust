fn main() {
    let (mut str1, str2) = two_words();
    join_words(&mut str1, &str2);
    println!("concatenated string is {:?}", str1);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

fn join_words(prefix: &mut String, suffix: &String) {
    prefix.push(' ');
    for ch in suffix.chars() {
        prefix.push(ch);
    }
}
