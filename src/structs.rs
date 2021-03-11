pub struct Data {
   pub num1: i32,
   pub num2: i32,
   pub option: Option<i32>
}

//tuple struct
pub struct MyTuple(pub i32, pub i32);

//placehoder for common functionality(class?)
pub struct Calculator;

//adding functions to a struct
impl Data {
    fn new() -> Self {
        Data {
            num1: 5,
            num2: 10,
            option: None
        }
    }
   fn sum(&self) -> i32 {
      &self.num1 + &self.num2
   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
       let data = Data::new();
        assert_eq!(data.sum(), 15);
    }

    #[test]
    fn sum_non_default_vals() {
        let mut data = Data::new();
        data.num1 = 1;
        assert_eq!(data.sum(), 11);
    }
}

