#[cfg(test)] // cfg! 테스트라는 단어를 찾아야 합니다.
mod testing {
    use super::*;

    #[test]
    fn check_if_five() {
        // bring_number() 함수는 5를 반환해야 합니다.
        assert_eq!(bring_number(true), 5);
    }
}

fn bring_number(should_run: bool) -> u32 {
    // 이 함수는 실행 여부를 나타내는 불리언을 받습니다.
    if cfg!(test) && should_run {
        // 실행되어야 하고 구성 테스트가 있으면 5를 반환합니다.
        5
    } else if should_run {
        // 테스트는 아니지만 실행해야 할 때 무언가를 출력합니다.
        // 테스트를 실행하면 println! 문을 무시합니다.
        println!("Returning 5. This is not a test");
        5
    } else {
        println!("This shouldn't run, returning 0."); // 그렇지 않으면 0을 반환합니다.
        0
    }
}

fn main() {
    bring_number(true);
    bring_number(false);
}
