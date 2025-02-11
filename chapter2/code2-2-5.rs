fn main() {
    let my_string = "'Ice to see you,' he said."; // 작은따옴표
    let quote_string = r#""Ice to see you," he said."#; // 큰따옴표
    // 문장 내부에 #이 있으므로 여닫는 부분에 ##을 사용했습니다.
    let many_hashtags = r####""You don't have to type ### to use a hashtag. You can just use #.""####;
    // 문장 내부에 ###(# 3개)이 있으므로 여닫는 부분에 ####(# 4개)을 사용했습니다.
    let many_hashtags = r####""You don't have to type ### to use a hashtag. You can just use #.""####;
    println!("{}\n{}\n{}\n{}\n", my_string, quote_string, hashtag_string, many_hashtags);
}
