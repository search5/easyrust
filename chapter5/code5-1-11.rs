use std::collections::HashMap;

fn main() {
    let data = vec![
        // 원시 데이터입니다.
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10)
    ];

    let mut survey_hash = HashMap::new();
    
    for item in data {
        // 이는 (&str, i32)의 튜플을 제공하고, 숫자를 벡터 내부로 밀어 넣습니다.
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }
    
    for (male_or_female, numbers) in survey_hash {
        println!("{male_or_female}: {numbers:?}");
    }
}
