fn main() {
    let my_string = String::from("I will go into the closure");
    let my_closure = || println!("{my_string}");
    
    my_closure();
    my_closure();
}
