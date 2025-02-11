fn main() {
    let one = true;
    let two = false;
    let three = true;
    let four = true;
    
    println!("{}", one && three); // true를 출력합니다.
    println!("{}", one && two && three && four); // false를 출력합니다.
}
