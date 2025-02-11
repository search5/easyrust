fn main() {
    let my_number = {
        let second_number = 8; // second_number 선언하고
        second_number + 9; // second_number를 더했지만 반환하지 않았습니다!
        // second_number는 더 이상 사용하지 않습니다.
    };
    println!("My number is: {:?}", my_number); // my_number는 ()와 같습니다.
}
