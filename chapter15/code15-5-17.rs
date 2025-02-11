trait SaysHello {
    fn hello(&self) {
        println!("Hello");
    }
}

impl<T> SaysHello for T {}

struct Nothing;

fn main() {
    8.hello();
    &'c'.hello();
    &mut String::from("Hello there").hello();
    8.7897.hello();
    Nothing.hello();
    std::collections::HashMap::<i32, i32>::new().hello();
}
