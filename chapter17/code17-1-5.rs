// ⚠
fn main() {
    let client = reqwest::blocking::Client::default();
    client.get("https://www.rust-lang.org").send();
}
