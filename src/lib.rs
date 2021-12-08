// pub mod parser;
pub mod database;
mod errors;
mod types;
mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{}", (4 + 4 - 1) & !(4 - 1));
    }
}
