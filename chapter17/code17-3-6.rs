use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::default();
    let response = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .unwrap();
    println!("{}", response.text().await.unwrap());
}
