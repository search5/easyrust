use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne {}

// 지금까지 Debug 트레이트를 구현한 오류 타입을 살펴봤습니다.
// 이제 Display 트레이트 구현을 알아보겠습니다.
impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!") // 이 메시지를 작성하기만 합니다.
    }
}

#[derive(Debug)] // ErrorTwo와 동일한 작업을 수행합니다.
struct ErrorTwo;

impl Error for ErrorTwo {}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

// 문자열이나 오류를 반환하는 함수를 만듭니다.
fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    // Box<dyn Error>를 사용하면 Error 트레이트가 있는 모든 타입을 반환할 수 있습니다.
    match input {
        0 => Err(Box::new(ErrorOne)), // Box에 꼭 넣으세요.
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me".to_string()), // success 타입입니다.
    }
}

fn main() {
    let vec_of_u8s = vec![0_u8, 1, 80]; // 시도해 볼 세 숫자입니다.

    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }
}
