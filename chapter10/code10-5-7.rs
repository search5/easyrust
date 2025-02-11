fn main() {
    let mut my_string = String::from("I will be changed in the closure");
    let mut my_closure = || {
        my_string.push_str(" now");
        println!("{my_string}");
    };

    my_closure();
    my_closure();
}
