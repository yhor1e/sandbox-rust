mod us_presidential_frontrunners {

    pub use self::democrats::HILLARY_CLINTON as democrat;
    pub use self::republicans::DONALD_TRUMP as republican;

    mod democrats {
        pub const HILLARY_CLINTON: &'static str = "Hillary Clinton";
        pub const BERNIE_SANDERS: &'static str = "Bernie Sanders";
    }

    mod republicans {
        pub const DONALD_TRUMP: &'static str = "Donald Trump";
        pub const JEB_BUSH: &'static str = "Jeb Bush";
    }

}

fn main() {
    println!(
        "candidates: {} and {}",
        us_presidential_frontrunners::democrat,
        us_presidential_frontrunners::republican
    );
}
