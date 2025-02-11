use std::error::Error as StdError;
use thiserror::Error;

#[derive(Error, Debug)]
enum SystemError {
    #[error("Couldn't send {0}")]
    SendError(i32),
    #[error("Something happened: {0}")]
    ExternalCrateError(String),
}

trait ToSystemError<T> {
    fn to_system_error(self) -> Result<T, SystemError>;
}

// Result<T, E>는 map_err가 Result<T, E>에서 Result<T, F>로 바뀌었기 때문입니다.
// T는 결과의 Ok 부분이고, E는 std::error::Error입니다.
// 그리고 F는 SystemError 열거형입니다.
impl<T, E: StdError> ToSystemError<T> for Result<T, E> {
    fn to_system_error(self) -> Result<T, SystemError> {
        self.map_err(|e| SystemError::ExternalCrateError(e.to_string()))
    }
}

fn parse_then_send(input: &[u8]) -> Result<(), SystemError> {
    let some_str = std::str::from_utf8(input).to_system_error()?;
    let number = some_str.parse::<i32>().to_system_error()?;
    send_number(number).to_system_error()?;
    Ok(())
}

fn send_number(number: i32) -> Result<(), SystemError> {
    if number < 1_000_000 {
        println!("Number sent!");
        Ok(())
    } else {
        println!("Too large!");
        Err(SystemError::SendError(number))
    }
}

fn main() {
    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));
}
