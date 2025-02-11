fn main() {
    // country라는 문자열이 있습니다.
    let country = String::from("Austria");
    // country_ref는 country 데이터에 대한 참조이며 변하지 않을 겁니다
    let country_ref = &country;
    // i8인 country라는 변수가 있습니다. 그러나 이는 country_ref와 관련이 없습니다.
    let country = 8;
    // country_ref는 여전히 String::from("Austria")의 데이터를 참조합니다.
    println!("{}, {}", country_ref, country);
}
