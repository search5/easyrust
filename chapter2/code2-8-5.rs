fn main() {
    // country는 변경할 수 없지만 Austria-Hungary를 출력할 것입니다.
    // 어떻게 했을까요?
    let country = String::from("Austria");
    adds_hungary(country);
}

// added_hungary 함수는 문자열을 가져와 변경 가능하다고 선언하면 됩니다.
fn adds_hungary(mut country: String) {
    country.push_str("-Hungary");
    println!("{}", country);
}
