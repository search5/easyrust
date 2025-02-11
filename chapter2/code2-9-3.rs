fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Kiribati");
    // 복제본을 만들어 함수에 제공하세요.
    // 복제본만 전달되고 country는 아직 살아 있습니다.
    prints_country(country.clone());
    prints_country(country);
}
