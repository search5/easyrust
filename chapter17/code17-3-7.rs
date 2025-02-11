use rand::*;

async fn wait_and_give_u8(num: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let wait_time = rng.gen_range(1..100);
    tokio::time::sleep(std::time::Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");

    num
}

#[tokio::main]
async fn main() {
    let num1 = wait_and_give_u8(1).await;
    let num2 = wait_and_give_u8(2).await;
    let num3 = wait_and_give_u8(3).await;
    
    println!("{num1}, {num2}, {num3}");
}
