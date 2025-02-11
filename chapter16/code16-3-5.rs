use std::fs;
use std::fs::File;
use std::io::Read; // .read_to_string() 함수를 사용하기 위해 작성했습니다.

fn main() -> std::io::Result<()> {
    fs::write("calvin_with_dad.txt",
    "Calvin: Dad, how come old photographs are always black and white? Didn't they
    have color film back then?
    Dad: Sure they did. In fact, those photographs *are* in color. It's just the
    *world* was black and white then.
    Calvin: Really?
    Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;

    let mut calvin_file = File::open("calvin_with_dad.txt")?; // 파일을 엽니다.
    let mut calvin_string = String::new(); // 이 문자열에 보관됩니다.
    
    calvin_file.read_to_string(&mut calvin_string)?; // 파일을 읽어 들입니다.
    calvin_string.split_whitespace().for_each(|word| print!("{} ", word.to_uppercase())); // 지금 문자열로 작업합니다.

    Ok(())
}
