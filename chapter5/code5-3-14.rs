fn main() {
    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or(&0);
    // .get이 동작하지 않으면 값을 &0으로 만듭니다.
    // .get은 참조를 반환하므로 0이 아닌 &0이 필요합니다.
    // fourth를 참조가 아닌 값으로 사용하고 싶다면 "let *fourth"와 같이 역참조할
    // 수 있습니다. 하지만 단순 출력이므로 여기서는 역참조가 필요하지 않습니다.
    println!("{fourth}");
}
