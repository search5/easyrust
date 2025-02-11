use std::collections::HashMap;

fn main() {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");

    if book_hashmap.get(&1).is_none() {
        // is_none()은 bool을 반환합니다.
        // None이면 true, Some이면 false를 반환합니다.
        book_hashmap.insert(1, "Le Petit Prince");
    }

    println!("{:?}", book_hashmap.get(&1));
}
