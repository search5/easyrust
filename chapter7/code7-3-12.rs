use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5]; // 이것은 Vec<i32>입니다.
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];
    // 이것은 Vec<&str>입니다.
    
    let number_word_hashmap: HashMap<_, _> = some_numbers // 여기서 타입을 알려 주므로
        .into_iter()
        .zip(some_words.into_iter())
        .collect(); // 여기서 타입을 알려 줄 필요가 없습니다.
}
