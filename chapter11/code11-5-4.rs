use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();
    
    sender.send(5);
    receiver.recv(); // recv는 receive의 약자입니다('rec v'가 아님).
}
