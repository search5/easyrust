// String의 소유권을 가져옵니다.
fn get_length(input: String) {
    // 단어 수를 계산하기 위해 분할합니다(공백은 단어 사이의 공백을 의미합니다).
    println!("It's {} words long.", input.split_whitespace().count());
}

fn main() {
    let mut my_string = String::new();
    for _ in 0..50 {
        // my_string에 문장을 밀어 넣습니다.
        my_string.push_str("Here are some more words ");
        // 매번 복제본을 제공합니다.
        get_length(my_string.clone());
    }
}
