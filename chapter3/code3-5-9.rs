fn match_colours(rbg: (i32, i32, i32)) {
    println!("Comparing a colour with {} red, {} blue, and {} green:", rbg.0, rbg.1, rbg.2);
    // 색상을 vec에 넣습니다. 내부에는 색상 이름이 있는 튜플이 있습니다.
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")];
    // true부터 시작하세요. 하나의 색상이 10 미만이면 false로 설정합니다.
    let mut all_have_at_least_10 = true;
    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false; // 지금은 거짓입니다.
            println!("Not much {}.", item.1); // 그리고 색상 이름을 출력합니다.
        }
    }
    if all_have_at_least_10 {
        // 여전히 true인지 확인하고 true이면 출력합니다.
        println!("Each colour has at least 10.");
    }
    println!(); // 한 줄 더 추가
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    match_colours(first);
    match_colours(second);
    match_colours(third);
}
