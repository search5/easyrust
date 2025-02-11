fn main() {
    let true_or_false = true;
    
    match true_or_false {
        true => println!("It's true"),
        false => println!("It's false"),
        true => println!("It's true"), // 앗, 또 true라고 썼네요.
    }
}
