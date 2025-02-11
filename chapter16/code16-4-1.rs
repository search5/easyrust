struct DoesNothing {}

struct PrintThing {}

impl PrintThing {
    fn prints_something() {
        println!("I am printing something");
    }
}

fn main() {}
