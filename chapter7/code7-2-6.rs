#[derive(Debug, Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            // 대부분 시립 도서관이므로 기본값을 City로 지정합니다.
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book) => Some(book + " is found!"),
            // 러스트는 String + &str을 허용합니다.
            None => None,
        }
    }
}

fn main() {
    let mut my_library = Library::new();
    
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    for item in my_library.clone() {
        // 이제 for 루프를 사용할 수 있습니다.
        // 도서관이 소멸되지 않도록 복제본을 제공하세요.
        println!("{}", item);
    }
}
