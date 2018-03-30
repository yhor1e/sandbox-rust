fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    // -----------------------------------------------------
    // 1. Make another, separate version of the data that's in `vec0` and pass that
    // to `fill_vec` instead.
    // ----------------------------------------------------
    //let mut vec1 = fill_vec(vec0.clone());

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    // -----------------------------------------------------
    // 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
    // and then copy the data within the function in order to return an owned
    // `Vec<i32>`
    // ----------------------------------------------------
    //let mut vec: Vec<i32> = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
