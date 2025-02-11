fn print_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name // 여기에서 반환합니다.
}

fn main() {
    let country = String::from("Austria");
    let country = print_country(country); // 문자열을 다시 가져오려면
    // 지금 여기에서 let을 사용해야 합니다.
    print_country(country);
}
