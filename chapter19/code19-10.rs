macro_rules! make_a_function {
    ($name:ident, $($input:tt),*) => {
        // 먼저 함수에 하나의 이름을 지정한 다음 다른 모든 것을 확인합니다.
        fn $name() {
            let output = stringify!($($input),*); // 다른 모든 것을 문자열로 만듭니다.
            println!("{}", output);
        }
    };
}

fn main() {
    // 우리가 지정한 모든 것을 출력하는 print_it()이라는 함수가 필요합니다.
    make_a_function!(print_it, 5, 5, 6, I);

    // 여기에서도 동일하지만 함수 이름을 변경합니다.
    make_a_function!(say_its_nice, this, is, really, nice);

    say_its_nice();
}
