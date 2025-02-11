macro_rules! might_print {
    ($input:expr) => {
        println!("You gave me: {:?}", $input); // 이제 {:?}를 사용하겠습니다.
        // 왜냐하면 다른 종류의 표현식을 제공하기 때문입니다.
    }
}

fn main() {
    might_print!(()); // ()를 제공합니다.
    might_print!(6); // 6을 제공합니다.
    might_print!(vec![8, 9, 7, 10]); // vec을 제공합니다.
}
