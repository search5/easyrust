use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();

    std::thread::spawn(move || {
        // sender로 보냅니다.
        sender.send("Send a &str this time").unwrap();
        sender.send("Send a &str this time").unwrap();
    });

    std::thread::spawn(move || {
        // sender_clone으로 보냅니다.
        sender_clone.send("And here is another &str").unwrap();
        sender_clone.send("And here is another &str").unwrap();
    });

    while let Ok(res) = receiver.recv() {
        println!("{res}");
    }
}
