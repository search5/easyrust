fn main() {
    let number_one = 6;
    let number_two = 10;
    
    let my_closure = || println!("{}", number_one + number_two);
    my_closure();
}
