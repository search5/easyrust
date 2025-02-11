fn main() {
    let str_vec = vec!["one", "two", "three"];
    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]); // 이들을 a, b, c라고 부름
    println!("{:?}", b);
}
