// ⚠
use reqwest::Client;

fn main() {
    let client = Client::default();
    client.get("https://www.rust-lang.org").send();
}
