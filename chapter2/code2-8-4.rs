// 함수가 변경 가능한 참조를 취한다고 알립니다.
fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary"); // push_str()은 문자열에 &str을 추가합니다.
    println!("Now it say: {}", country_name);
}

fn main() {
    let mut country = String::from("Austria");
    add_hungary(&mut country); // 변경 가능한 참조를 제공해야 합니다.
}
