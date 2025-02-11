fn main() {
    let number_one = 6;
    let number_two = 10;
    let my_closure = |x: i32| println!("{}", number_one + number_two + x);
    my_closure(5);
}
