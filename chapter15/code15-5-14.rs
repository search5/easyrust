use thiserror::Error;
use SystemError::*;

#[derive(Error, Debug)]
enum SystemError {
    #[error("Couldn't parse into a str: {0}")]
    StringFromUtf8Error(String),
    #[error("Couldn't turn into an i32: {0}")]
    ParseI32Error(String),
    #[error("Couldn't send: {0}")]
    SendError(i32),
    #[error("Something happened")]
    OtherError,
}

fn parse_then_send(input: &[u8]) -> Result<(), SystemError> {
    let some_str = std::str::from_utf8(input).map_err(|e| StringFromUtf8Error(e.to_string()))?;
    let number = some_str
        .parse::<i32>()
        .map_err(|e| ParseI32Error(e.to_string()))?;
    send_number(number).map_err(|e| SendError(e.to_string()))?;
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
