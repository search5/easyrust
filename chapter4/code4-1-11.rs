use std::fmt::Display;

fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
    // 타입 T와 U에는 Display가 필요합니다.
    println!("I have two things to say: {statement_1} and {statement_2}");
}

fn main() {
    say_two("Hello there!", String::from("I hate sand."));
    // 타입 T는 &str이지만 타입 U는 문자열입니다.
    say_two(String::from("Where is Padme?"), String::from("Is she all right?"));
    // 두 타입 모두 문자열입니다.
}
