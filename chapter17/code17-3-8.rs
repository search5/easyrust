use rand::*;
use tokio;
use tokio::join;

async fn wait_and_give_u8(num: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let wait_time = rng.gen_range(1..100);
    tokio::time::sleep(std::time::Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");

    num
}

#[tokio::main]
async fn main() {
    let nums = join!(
        wait_and_give_u8(1),
        wait_and_give_u8(2),
        wait_and_give_u8(3)
    );

    println!("{nums:?}");
}
