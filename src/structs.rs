pub struct Data {
   pub num1: i32,
   pub num2: i32,
   pub option: Option<i32>
}

//tuple struct
pub struct MyTuple(pub i32, pub i32);

//placehoder for common functionality(class?)
pub struct Calculator;

impl Calculator {
    fn add(n1: i32, n2: i32)-> i32 {
        // no Self reference because Calculator doesn't have a body to self reference
        n1 + n2
    }
    fn div(n1: i32, n2: i32)-> f32 {
        // casting
        ( n1 / n2 ) as f32
    }
}

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
    
    #[test]
    fn populate_struct_with_vals_from_other_struct() {
       let s1 = Data {
           num1: 99,
           num2: 99,
           option: Some(1)
       }; 
        
        let s2 = Data { num1:1, ..s1 };
        assert_eq!(s2.num2, 99);
        assert_eq!(s2.option, Some(1));
    }

    #[test]
    fn calculator_sum() {
        assert_eq!(Calculator::add(1,2), 3);
        assert_eq!(Calculator::div(5,2), 2.0);
    }
}

