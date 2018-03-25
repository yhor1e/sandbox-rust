fn main() {
    call_me();
}

fn call_me() {
    let num = 3;
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
