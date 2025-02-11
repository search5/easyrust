#![no_implicit_prelude]
extern crate std; // std 크레이트 사용을 명시적으로 선언
use std::vec; // vec 매크로 사용을 위한 import
use std::string::String; // String 타입 사용을 위한 import
use std::convert::From; // &str에서 String으로의 변환을 위한 트레이트 import
use std::println; // 출력 매크로 사용을 위한 import

fn main() {
    let my_vec = vec![8, 9, 10];
    let my_string = String::from("This won't work");
    println!("{:?}, {}", my_vec, my_string);
}
