fn main() {
    let a = [1, 2, 3, 4, 5];
    let nice_slice = [a[1], a[2], a[3]];

    if nice_slice == [2, 3, 4] {
        println!("Nice slice!");
    } else {
        println!("Not quite what I was expecting... I see: {:?}", nice_slice);
    }
}
