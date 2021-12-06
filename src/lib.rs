// pub mod parser;
mod errors;
mod schemes;
mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{}", (4 + 4 - 1) & !(4 - 1));
    }
}
