struct File(String);

fn main() {
    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
    
    println!("{}", my_file.0 == my_string);
    // my_file.0은 String이므로 true를 출력합니다.
}
