use std::sync::Mutex;
use std::thread;

fn main() {
    let my_number = Mutex::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });

        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
    });
}
