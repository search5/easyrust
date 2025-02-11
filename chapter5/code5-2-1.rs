use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?; // 여기에 물음표가 있습니다.
    Ok(parsed_number)
}

fn main() {}
