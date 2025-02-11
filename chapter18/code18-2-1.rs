fn main() {
    let korean_word = "청춘예찬";
    for character in korean_word.chars() {
        print!("{} ", character.escape_unicode());
    }
}
