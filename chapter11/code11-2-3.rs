use std::fmt::Display;

fn prints_it(input: impl Into<String> + Display) {
    // String으로 바뀔 수 있고 Display가 있는 모든 것을 취합니다.
    println!("You can print many things, including {input}");
}

fn main() {
    let name = "Tuon";
    let string_name = String::from("Tuon");
    prints_it(name);
    prints_it(string_name);
}
