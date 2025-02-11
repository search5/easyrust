use std::fmt::Display;

fn print_and_return_thing<T: Display>(input: T) -> T {
    println!("You gave me {input} and now I will give it back.");
    input
}

fn main() {
    let my_name = print_and_return_thing("Windy");
    let small_number = print_and_return_thing(9.0);
}
