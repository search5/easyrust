use std::ffi::OsString;

fn main() {
    // ⚠
    let os_string = OsString::from("This string works for your OS too.");
    match os_string.into_string() {
        Ok(valid) => valid.thth(),  // 컴파일러: ".thth()가 뭐야??"
        Err(not_valid) => not_valid.occg(), // 컴파일러: ".occg()가 뭐야??"
    }
}
