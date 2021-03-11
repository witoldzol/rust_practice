enum Color {
    Red,
    Green,
    Blue,
}

enum Number {
    //primitives
    One,
    Two,
    // struct variance
    //tuple struct
    Custom(String),
    //classic struct
    Coord { x: i32, y: i32 },
}

pub struct Data {
    pub num1: i32,
    pub num2: i32,
    pub option: Option<i32>,
}

struct StringData {
    s: String,
}

//tuple struct
pub struct MyTuple(pub i32, pub i32);

//placehoder for common functionality(class?)
pub struct Calculator;

impl Calculator {
    fn add(n1: i32, n2: i32) -> i32 {
        // no Self reference because Calculator doesn't have a body to self reference
        n1 + n2
    }
    fn div(n1: i32, n2: i32) -> f32 {
        // casting
        (n1 / n2) as f32
    }
}

//adding functions to a struct
impl Data {
    fn new() -> Self {
        Data {
            num1: 5,
            num2: 10,
            option: None,
        }
    }
    fn sum(&self) -> i32 {
        &self.num1 + &self.num2
    }
}

trait Tranform {
    fn rev(&self) -> String;

    fn print_rev(&self) {
        eprintln!("&self.rev() = {:?}", &self.rev());
    }
}

impl Tranform for StringData {
    fn rev(&self) -> String {
        self.s.chars().rev().collect::<String>()
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
            option: Some(1),
        };

        let s2 = Data { num1: 1, ..s1 };
        assert_eq!(s2.num2, 99);
        assert_eq!(s2.option, Some(1));
    }

    #[test]
    fn calculator_sum() {
        assert_eq!(Calculator::add(1, 2), 3);
        assert_eq!(Calculator::div(5, 2), 2.0);
    }

    #[test]
    fn trait_reverse_value() {
        let s = StringData {
            s: "YUP".to_string(),
        };
        assert_eq!(s.rev(), "PUY");
        assert_eq!(s.s, "YUP");
        s.print_rev();
    }

    #[test]
    fn color() {
        let favourite: Color = Color::Red;
        //&str is a string slice  - it's immutable
        //String is a HEAP allocated string - it can me mutated (if mut set)
        let custom = Number::Custom("one pierdyliard".to_string());

        match favourite {
            Color::Red => println!("its a red color"),
            Color::Blue => println!("blue"),
            _ => println!("some other color"),
        }

        match custom {
            Number::Custom(v) => eprintln!("v = {:?}", v),
            _ => {}
        }

        if let Color::Red = favourite {
            println!("its red")
        }
    }

    #[test]
    fn test_option() {
        let mut age: Option<i32> = None;
       age = Some(11);

        assert_eq!(age.unwrap(), 11);

        match age  {
            Some(11) => println!("I'm eleven!!!"),
            None => println!("NOTHING!"),
            _ => println!("I'm NOT eleven")
        }

        if let Some(11) = age {
            println!("IF LET IS FAST!");
        }

        if let Some(v) = age {
            println!("i've passed value through = {}", v * 2);
        }

        println!("did age mutated ? {}", age.unwrap());
    }
}
