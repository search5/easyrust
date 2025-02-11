mod print_things {
    use std::fmt::Display;
    
    fn prints_one_thing<T: Display>(input: T) { // Display를 구현하는 모든 항목을 출력합니다.
        println!("{}", input)
    }
}

fn main() {}
