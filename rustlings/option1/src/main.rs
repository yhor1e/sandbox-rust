fn main() {
    let mut list = vec![3];

    let last = list.pop().unwrap_or(0);
    println!("The last item in the list is {:?}", last);

    let second_to_last = list.pop().unwrap_or(0);
    println!(
        "The second-to-last item in the list is {:?}",
        second_to_last
    );
}
