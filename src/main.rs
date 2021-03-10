use std::{error, result};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

fn read_file(path: &str) -> TResult<String> {
    unimplemented!();
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file(){
        let result = read_file("test_data/test_one");
        assert!(result.is_ok());
    }
}
