use std::io::{Error, ErrorKind};

fn parse_then_send(input: &[u8]) { // 반환 타입은 무엇인가요?
    let some_str = std::str::from_utf8(input)?;
    let number = some_str.parse::<i32>()?;
    send_number(number)?;
}

fn send_number(number: i32) -> Result<(), Error> {
    if number < 1_000_000 {
        println!("Number sent!");
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidData))
    }
}

fn main() {}
