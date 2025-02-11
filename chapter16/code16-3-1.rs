use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 다음 이름으로 파일을 만듭니다.
    // 조심하세요. 이미 이 이름의 파일이 있으면 내용이 모두 삭제됩니다.
    let mut file = fs::File::create("myfilename.txt")?;
    
    // " 앞의 b를 잊지 마세요. 파일은 바이트를 사용하기 때문입니다.
    file.write_all(b"Let's put this in the file")?;
    Ok(())
}
