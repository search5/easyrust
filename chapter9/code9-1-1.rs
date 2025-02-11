fn prints_str(my_str: &str) { // 러스트는 &String을 &str로 변환하여 사용합니다.
    println!("{my_str}");
}

fn main() {
    let my_string = String::from("I am a string");
    prints_str(&my_string); // prints_str에 &String을 제공합니다.
}
