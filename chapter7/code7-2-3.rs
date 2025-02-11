#[derive(Debug)] // {:?}로 출력하고 싶습니다.
struct Library {
    library_type: LibraryType, // 우리가 만든 열거형입니다.
    books: Vec<String>,        // 책 목록
}

#[derive(Debug)]
enum LibraryType {
    // 도서관은 시립 도서관이나 국가 도서관이 될 수 있습니다.
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        // add_book을 사용해 새 책을 추가합니다.
        self.books.push(book.to_string());
        // &str을 가져와 문자열로 바꾼 다음 Vec에 추가합니다.
    }
    fn new() -> Self {
        // 새로운 도서관을 생성합니다.
        Self {
            library_type: LibraryType::City,
            // 대부분 시립 도서관이므로 기본값을 City로 지정합니다.
            books: Vec::new(),
        }
    }
}

fn main() {
    let mut my_library = Library::new(); // 새 도서관을 생성합니다.
    my_library.add_book("The Doom of the Darksword"); // 몇 권의 책을 추가합니다.
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");
    
    println!("{:?}", my_library.books); // 도서 목록을 출력할 수 있습니다
}
