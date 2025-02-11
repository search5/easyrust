fn math(input: &str) -> i32 {
    6
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }
    
    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!(math("1 - 2"), -1);
    }
    
    #[test]
    fn one_minus_minus_one_is_two() {
        assert_eq!(math("1 - -1"), 2);
    }
}
