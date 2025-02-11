fn main() {
    let a_string = "I don't have any dashes in me.";

    println!(
        "{}",
        a_string
            .chars() // 이제 이터레이터입니다.
            .fold("-".to_string(), |mut string_so_far, next_char| {
                // 문자열 -로 시작합니다.
                // next_char 문자와 함께 매번 변경 가능한 값을 가져옵니다.
                string_so_far.push(next_char); // 문자를 추가한 다음 '-'을 넣습니다.
                string_so_far.push('-');
                string_so_far} // 다음 루프로 전달하는 것을 잊지 마세요.
        ));
}
