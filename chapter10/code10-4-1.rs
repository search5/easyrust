fn takes_a_string(input: String) {
    println!("It is: {input}")
}

fn also_takes_a_string(input: String) {
    println!("It is: {input}")
}

fn main() {
    let user_name = String::from("User MacUserson");

    takes_a_string(user_name);
    also_takes_a_string(user_name); // ⚠
}
