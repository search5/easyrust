fn main() {
    let my_closure = |x: i32| println!("{x}");
    my_closure(5);
    my_closure(5 + 5);
}
