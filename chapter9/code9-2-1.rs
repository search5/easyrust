fn returns_reference() -> &str {
    let my_string = String::from("I am a string");
    &my_string // ⚠
}

fn main() {}
