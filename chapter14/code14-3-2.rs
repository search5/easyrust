const OKAY_CHARACTERS: &str = "1234567890+- "; // 마지막에 있는 공백을 잊지 마세요.

fn math(input: &str) -> i32 {
    if !input.chars().all(|character| OKAY_CHARACTERS.contains(character)) {
        panic!("Please only input numbers, +-, or spaces");
    }
    6 // 여전히 6을 반환합니다.
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
    
    #[test]
    #[should_panic] // 새로운 테스트는 다음과 같으며 패닉이 발생해야 합니다.
    fn panics_when_characters_not_right() {
        math("7 + seven");
    }
}
