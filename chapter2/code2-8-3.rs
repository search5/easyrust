fn print_country(country_name: &String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Austria");
    print_country(&country); // "Austria"를 출력합니다.
    print_country(&country); // 다시 한번 해 봅시다!
}
