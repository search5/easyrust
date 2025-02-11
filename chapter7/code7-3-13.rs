fn main() {
    let numbers_together = "140399923481800622623218009598281";
    
    for (index, number) in numbers_together.char_indices() {
        match (index % 3, number) {
            (0..=1, number) => print!("{}", number),
            // 나머지가 있으면 숫자를 출력하세요.
            _ => print!("{}\t", number),
            // 그렇지 않으면 숫자 다음에 탭 공백을 출력하세요.
        }
    }
}
