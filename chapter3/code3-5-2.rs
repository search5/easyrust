fn main() {
    let mut counter = 0; // counter를 0으로 설정합니다.
    loop {
        counter += 1; // counter를 1 증가시킵니다.
        println!("The counter is now: {counter}");
        if counter == 5 {
            // counter == 5가 되면 중단합니다.
            break;
        }
    }
}
