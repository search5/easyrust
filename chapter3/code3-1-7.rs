fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let mut my_vec = Vec::new();
    // 이 시점에 프로그램을 실행하면 컴파일러에서 오류가 발생합니다.
    // Vec의 타입을 모릅니다.
    my_vec.push(name1); // Vec<String>임을 이제 압니다.
    my_vec.push(name2);
}
