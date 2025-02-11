fn main() {
    // vec!를 추가한 것을 제외하고는 앞의 코드와 동일합니다.
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];
    println!(
        "Three to five: {:?},\nstart at two: {:?},\nend at five: {:?},\neverything: {:?}",
        three_to_five, start_at_two, end_at_five, everything
    );
}
