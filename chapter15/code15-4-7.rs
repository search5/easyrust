struct User {
    name: String,
    number: u32,
}

fn main() {
    println!("{}", std::mem::size_of::<User>());
}
