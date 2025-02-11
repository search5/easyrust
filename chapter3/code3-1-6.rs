fn main() {
    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &array_of_ten[2..5]; // 인덱스 2에서 5까지(인덱스 5 제외)
    let start_at_two = &array_of_ten[1..]; // 인덱스 2에서 끝까지
    let end_at_five = &array_of_ten[..5]; // 처음부터 인덱스 5까지
    let everything = &array_of_ten[..]; // 전체 배열
    println!(
        "Three to five: {:?}, start at two: {:?}, end at five: {:?}, everything: {:?}",
        three_to_five, start_at_two, end_at_five, everything
    );
}
