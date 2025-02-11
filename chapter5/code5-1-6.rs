use std::collections::HashMap;

fn main() {
    // "Eye of the World"가 두 번 나타납니다.
    let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"];

    let mut book_hashmap = HashMap::new();
    
    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
    
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {book}? {true_or_false}");
    }
}
