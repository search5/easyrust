use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5]; // 이것은 Vec<i32>입니다.
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];
    // 이것은 Vec<&str>입니다.

    let number_word_hashmap = some_numbers
        .into_iter() // 이제 이터레이터입니다.
        .zip(some_words.into_iter()) // .zip() 내부에 다른 이터레이터를 넣었습니다.
                                     // 이제 이들은 함께 연결되었습니다.
        .collect::<HashMap<_, _>>();
    
    println!(
        "For key {} we get {}.",
        2,
        number_word_hashmap.get(&2).unwrap()
    );
}
