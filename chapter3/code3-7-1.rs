// 사람을 나타내는 간단한 구조체 만들기
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

// papa_doc 변수 생성
fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    // papa_doc 변수 해체
    let Person { name: a, real_name: b, height: c, happiness: d } = papa_doc;
    println!("They call him {a} but his real name is {b}. \
He is {c} cm tall and is he happy? {d}");
}
