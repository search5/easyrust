use std::time::Duration;
use std::thread::sleep;

fn main() {
    let three_seconds = Duration::from_secs(3);
    println!("I must sleep now.");
    sleep(three_seconds);
    println!("Did I miss anything?");
}
