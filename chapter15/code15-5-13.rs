use thiserror::Error;
use SystemError::*;

#[derive(Error, Debug)]
enum SystemError<'a> {
    #[error("Couldn't parse {0:?} into a str")]
    StringFromUtf8Error(&'a [u8]),
    #[error("Couldn't turn {0} into an i32")]
    ParseI32Error(&'a str),
    #[error("Couldn't send {0}; number is too large")]
    SendError(i32),
    #[error("Something happened")]
    OtherError,
}

fn parse_then_send(input: &[u8]) -> Result<(), SystemError> {
    let some_str = std::str::from_utf8(input).map_err(|_| StringFromUtf8Error(input))?;
    let number = some_str
        .parse::<i32>()
        .map_err(|_| ParseI32Error(some_str))?;
    send_number(number)?;
    Ok(())
}

fn send_number<'a>(number: i32) -> Result<(), SystemError<'a>> {
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
