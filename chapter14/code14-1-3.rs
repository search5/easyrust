// ⚠
mod print_things {
    use std::fmt::Display;

    fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input)
    }
}

fn main() {
    use print_things::prints_one_thing;
    
    prints_one_thing(6);
    prints_one_thing("Trying to print a string...".to_string());
}
