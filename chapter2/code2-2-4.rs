fn main() {
    println!(
        "He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file."
    ); // 여기서 \를 다섯 번 사용했습니다.
    println!(
        r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
    ) // 훨씬 낫습니다!
}
