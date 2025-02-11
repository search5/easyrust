fn main() {
    let mut my_string = String::from("Can I go inside the thread?");
    
    let handle = std::thread::spawn(|| {
        println!("{my_string}"); //
    });

    handle.join();
}
