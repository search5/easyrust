fn main() {
    // 다음은 i32 타입입니다.
    let my_number = 8;
    println!("{}", my_number); // 8을 출력합니다.
    {
        // 다음은 f64 타입입니다. 앞의 my_number가 아니며 완전히 다른 변수입니다.
        let my_number = 9.2;
        println!("{}", my_number); // 9.2를 출력합니다.
        // 그러나 이곳의 my_number는 이 블록이 끝날 때까지만 존재합니다.
        // 앞선 my_number는 아직 살아 있습니다!
    }
    println!("{}", my_number); // 9.2가 아니라 8을 출력합니다.
}
