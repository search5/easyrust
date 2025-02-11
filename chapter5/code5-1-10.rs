use std::collections::HashMap;

fn main() {
    let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of theWorld", "Eye of the World"];
    
    let mut book_hashmap = HashMap::new();
    
    for book in book_collection {
        // return_value는 변경 가능한 참조입니다. 아무것도 없으면 0이 됩니다.
        let return_value = book_hashmap.entry(book).or_insert(0);
    
        // 이제 return_value는 1 이상입니다. 다른 책이 있으면 1씩 올라갑니다.
        *return_value += 1;
    }
    
    for (book, number) in book_hashmap {
        println!("{book}, {number}");
    }
}
