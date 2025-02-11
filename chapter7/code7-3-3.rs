fn main() {
    let my_closure = || {
        let number = 7;
        let other_number = 10;
        println!("The two numbers are {number} and {other_number}.");
        // 이 클로저는 함수처럼 원하는 만큼 코드를 길게 쓸 수 있습니다.
    };
    
    my_closure();
}
