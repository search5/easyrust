// 이 반환 타입은 매우 깁니다.
fn returns<’a>(
    input: &’a Vec<char>,
) -> std::iter::Take<std::iter::Skip<std::slice::Iter<’a, char>>> {
    input.iter().skip(4).take(5)
}

fn main() {}
