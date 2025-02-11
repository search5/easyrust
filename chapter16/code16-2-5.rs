use std::env::args;

fn main() {
    let input = args();
    input.skip(1).for_each(|item| {
        println!("You wrote {}, which in capital letters is {}", item, item.to_uppercase());
    })
}
