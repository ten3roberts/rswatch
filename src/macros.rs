#[macro_use]
macro_rules! verb_print {
    ($verbose:ident, $($arg:tt)*) => {
        if $verbose {
        println!($($arg)*);
        }
    }
}

#[macro_use]
macro_rules! error {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::process::exit(1);
    })

}
#[macro_use]
macro_rules! warn {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
    })

}
