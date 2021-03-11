#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_value_from_if_statement() {
        let result = if 5 > 3
        {10}
        else {0};

        assert_eq!(result, 10);

        match result {
            x if x == 10 => println!("its exactly {}", x),
            0..=10 => println!("its between 0 ad 10"),
            _ => {}
        }
    }

    #[test]
    fn return_from_loop() {
        let mut i = 0;

        let result = loop {
            if i == 99 {
                break i;
            }

            i += 1;
        };
        assert_eq!(result, 99);
    }
    
    #[test]
    fn for_loop() {
        let mut i = 0;
        // .step_by(1) is DEFAULT - no need to specify
        for x  in (0..10).step_by(1) {
            i += 2;
        }
        assert_eq!(i, 20);
    }
}
