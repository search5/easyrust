fn main() {
    let helpful_message = if cfg!(target_os = "windows") { "backslash" } else { "slash" };
    
    println!(
        "...then in your hard drive, type the directory name followed by a {}. \
        Then you...",
        helpful_message
    );
}
