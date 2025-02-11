#[derive(Debug)]
struct City<'city> { // 수명은 이제 'city라고 불립니다.
    name: &'city str, // name도 'city 수명을 가집니다.
    date_founded: u32,
}

fn main() {}
