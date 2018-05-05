fn main() {
    Print::print(&123456);
    Print::print(&'💕');
}

trait Print {
    fn print(&self);
}

impl Print for u64 {
    fn print(&self) {
        println!("Print for u64: {}", self);
    }
}

impl Print for char {
    fn print(&self) {
        println!("Print for char: {}", self);
    }
}
