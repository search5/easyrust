use std::collections::HashMap;

fn main() {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");
    
    println!("{:?}", book_hashmap.get(&1));
    // .get()은 참조를 취하므로 여기에 &1을 넣었습니다.
}
