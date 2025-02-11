fn main() {
    let mut my_string = String::from("Can I go inside the thread?");
    let handle = std::thread::spawn(|| {
       println!("{my_string}"); // 이제 my_string이 참조로 사용됩니다.
    });

    drop(my_string); // ⚠ 여기에서 버리려고 합니다.
    // 그러나 스레드에는 여전히 my_string이 필요합니다.
    
    handle.join();
}
