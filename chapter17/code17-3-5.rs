use tokio;

async fn async_give_8() -> u8 {
    8
}

#[tokio::main]
async fn main() {
    let some_number = async_give_8().await;
}
