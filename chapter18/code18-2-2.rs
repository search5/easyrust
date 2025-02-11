use std::convert::TryFrom; // TryFrom을 사용하려면 이렇게 해야 합니다.
use rand::prelude::*; // 난수도 사용할 것입니다.

fn main() {
    let some_character = char::from(99); // 간단한 방법이며 TryFrom이 필요 없습니다.
    println!("{}", some_character);

    let mut random_generator = rand::thread_rng();
    // 이 함수는 40,000번 시도해 u32에서 문자를 만듭니다.
    // 범위는 0(std::u32::MIN)에서 u32의 가장 큰 숫자(std::u32::MAX)까지입니다.
    // 실패하면 '-'를 반환합니다.
    
    for _ in 0..40_000 {
        let bigger_character = char::try_from(random_generator
            .gen_range(std::u32::MIN..std::u32::MAX)).unwrap_or('-');
        print!("{}", bigger_character)
    }
}
