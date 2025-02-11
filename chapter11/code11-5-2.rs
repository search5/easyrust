use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel(); // ⚠
}
