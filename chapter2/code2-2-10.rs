fn main() {
    println!("{:X}", '행' as u32); // 16진숫값을 얻으려면 u32로 타입 변환하세요.
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");
    // 유니코드 이스케이프 문자인 \u로 출력해 보세요.
}
