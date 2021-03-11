#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector() {
        let mut nums :Vec<i32> = Vec::new();
        nums.push(1);
        nums.push(2);
        let nums2 = vec![1,2];
        assert_eq!(nums, nums2);
    }
}
