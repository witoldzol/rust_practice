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
            0..=10 => println!("its between 0 ad 10"),
            _ => {}
        }
    }
}
