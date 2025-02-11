fn main() {
    let my_string = String::from("Can I go inside the thread?");
    
    let handle = std::thread::spawn(move|| {
        println!("{my_string}");
    });

    handle.join().unwrap();
}
