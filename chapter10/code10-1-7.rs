struct File(String); // File은 String을 둘러싼 래퍼입니다.

fn main() {
    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
    println!("{}", my_file == my_string); // ⚠ File과 String을 비교할 수 없습니다.
}
