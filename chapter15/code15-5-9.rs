use anyhow::{anyhow, Result};

fn parse_then_send(input: &[u8]) -> Result<()> {
    let some_str = std::str::from_utf8(input)?;
    let number = some_str.parse::<i32>()?;
    send_number(number)?;
    Ok(())
}

fn send_number(number: i32) -> Result<()> {
    if number < 1_000_000 {
        println!("Number sent!");
        Ok(())
    } else {
        println!("Too large!");
        Err(anyhow!("Number is too large"))
    }
}

fn main() {
    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));
}
