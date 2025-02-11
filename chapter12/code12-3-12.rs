fn handle_error_inside_function() {
    println!("{:?}", "seven".parse::<i32>());
}

fn main() {
    handle_error_inside_function();
}
