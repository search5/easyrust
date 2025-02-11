fn main() {
    let client = reqwest::blocking::Client::default();
    let response = client.get("https://www.rust-lang.org").send().unwrap();
    println!("{}", response.text().unwrap());
}
