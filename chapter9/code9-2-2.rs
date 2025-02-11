fn returns_str() -> &str {
    let my_string = String::from("I am a string");
    "I am a str" // ⚠
}

fn main() {
    let my_str = returns_str();
    println!("{my_str}");
}
