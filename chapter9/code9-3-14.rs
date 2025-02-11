use std::sync::RwLock;

fn main() {
    let my_rwlock = RwLock::new(5);
    
    let read1 = my_rwlock.read().unwrap(); // 하나의 .read()는 괜찮습니다.
    let read2 = my_rwlock.read().unwrap(); // 두 개의 .read()도 괜찮습니다.
    
    println!("{:?}, {:?}", read1, read2);
    
    let write1 = my_rwlock.write().unwrap(); // 이제 프로그램은 영원히 기다릴 겁니다.
}
