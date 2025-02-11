use std::fs::{read_to_string, write, File};
use std::io::Write;

fn main() -> std::io::Result<()> {
    write("calvin_with_dad.txt",
    "Calvin: Dad, how come old photographs are always black and white? Didn't they
    have color film back then?
    Dad: Sure they did. In fact, those photographs *are* in color. It's just the
    *world* was black and white then.
    Calvin: Really?
    Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;

    let mut calvin_file = File::options()
        .append(true) // 이제 삭제하지 않고 작성할 수 있습니다.
        .read(true)
        .open("calvin_with_dad.txt")?;
    
    calvin_file.write_all(b"And it was a pretty grainy color for a while too.\n")?;
    write!(&mut calvin_file, "That's really weird.\n")?;
    write!(&mut calvin_file, "Well, truth is stranger than fiction.")?;
    
    println!("{}", read_to_string("calvin_with_dad.txt")?);
    
    Ok(())
}
