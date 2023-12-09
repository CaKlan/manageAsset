
#[macro_export]
macro_rules! cls {
    () => {
        print!("{}[2J", 27 as char)
    };
}

#[macro_export]
macro_rules! flush {
    () => {
        use std::io::Write;
        std::io::stdout().flush().unwrap()
    };
}