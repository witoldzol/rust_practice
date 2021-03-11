use std::fs::read_to_string;
use std::{error, result};
mod structs;
mod control_flow;
mod vectors;

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

fn sum_vector(input: &Vec<i32>) -> i32 {
    input.iter().fold(0, |mut sum, curr| {
        sum += curr;
        sum
    })
}

fn split_to_vector(input: &String) -> TResult<Vec<usize>> {
    input
        .trim()
        .replace("\n", ",")
        .split(',')
        .map(|x| x.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

fn read_file(path: &str) -> TResult<String> {
    read_to_string(path).map_err(|e| e.into())
}

fn main() {
    let a = structs::Data {
        num1: 1,
        num2: 3,
        option: None,
    };
    let t = structs::MyTuple(1, 2);
    eprintln!("tuple.1 = {:?}, tuple.2 = {:?}", t.0, t.1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file() {
        let result = read_file("test_data/test_one");
        assert!(result.is_ok());
    }

    #[test]
    fn returns_12345_string() {
        let result = read_file("test_data/test_one");
        let expected = "1,2,3,4,5\n".to_string();
        if let Ok(result) = result {
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn returns_error_when_file_not_found() {
        let result = read_file("test_data/empty_2");
        if let Err(result) = result {
            assert!(result.to_string().contains("No such file or directory"))
        } else {
            assert!(false);
        }
    }

    #[test]
    fn split_numbers_to_vector() {
        let input = String::from("1,2,3,4,5\n");
        let expected = vec![1, 2, 3, 4, 5];
        let result = split_to_vector(&input);

        if let Ok(result) = result {
            assert_eq!(result, expected);
        }
    }
    #[test]
    fn split_multiline_numbers_to_vector() {
        let input = String::from("\r1\n2\n3\n\n");
        let expected = vec![1, 2, 3];
        let result = split_to_vector(&input);

        if let Ok(vector) = result {
            assert_eq!(vector, expected);
        } else {
            panic!();
        }
    }

    #[test]
    fn sum_all_the_numbers_in_vector() {
        let sum_5 = sum_vector(&vec![1, 2, 3, 0, -1]);
        let sum_99 = sum_vector(&vec![-100, 0, 199]);
        let sum_minus_10 = sum_vector(&vec![-100, 0, 90]);
        let sum_none = sum_vector(&vec![]);

        assert_eq!(sum_5, 5);
        assert_eq!(sum_99, 99);
        assert_eq!(sum_minus_10, -10);
        assert_eq!(sum_none, 0);
    }
}
