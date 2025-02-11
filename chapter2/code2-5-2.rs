fn return_str() -> &str {
    let country = String::from("Austria");
    let country_ref = &country;
    country_ref // ⚠
}

fn main() {
    let country = return_str();
}
