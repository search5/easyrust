struct File(String); // File은 String을 둘러싼 래퍼입니다.

fn main() {
    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
}
