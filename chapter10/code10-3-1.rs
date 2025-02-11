// 자, 먼저 Book 구조체가 필요합니다.
// 아직 아무것도 없고 나중에 추가할 예정입니다.
struct Book {}

enum BookType {
    // 책은 하드 커버나 소프트 커버일 수 있으므로 열거형을 추가합니다.
    HardCover,
    SoftCover,
}

// ⚠ get_book은 &Book을 가져가서 Option<String>을 반환해야 합니다.
fn get_book(book: &Book) -> Option<String> {}

// delete_book은 Book 참조를 가져가서 Result를 반환해야 합니다.
// 할 일: impl 블록은 이러한 함수를 메서드로 만듭니다.
// 할 일: 이를 적절한 오류로 만드세요.
fn delete_book(book: &Book) -> Result<(), String> {}

fn check_book_type(book_type: &BookType) {
    // match 문이 동작하는지 확인합니다.
    match book_type {
        BookType::HardCover => println!("It's hardcover"),
        BookType::SoftCover => println!("It's softcover"),
    }
}

fn main() {
    let book_type = BookType::HardCover;
    check_book_type(&book_type); // 좋아요, 이 함수를 확인해 봅시다!
}
