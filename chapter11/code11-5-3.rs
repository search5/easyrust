use std::sync::mpsc::{channel, Sender, Receiver}; // Sender와 Receiver를 추가했습니다.

fn main() {
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = channel();
}
