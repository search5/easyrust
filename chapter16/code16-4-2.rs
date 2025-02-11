/// 이것은 아무것도 하지 않는 구조체입니다.
struct DoesNothing {}

/// 이 구조체에는 메서드가 하나만 있습니다.
struct PrintThing {}

/// 항상 같은 메시지를 출력합니다.
impl PrintThing {
    fn prints_something() {
        println!("I am printing something");
    }
}

fn main() {}
