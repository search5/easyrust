use std::time::Instant;

fn main() {
    let time1 = Instant::now();
    let time2 = Instant::now(); // 두 변수의 시점은 매우 가깝습니다.
    
    let mut new_string = String::new();

    loop {
        // 러스트가 이 조지아 문자를 문자열에 밀어 넣도록 합니다.
        new_string.push('წ');
        if new_string.len() > 100_000 { // 문자열의 크기가 100,000 바이트가 될 때까지 반복합니다.
            break;
        }
    }

    let time3 = Instant::now();

    println!("{:?}", time2 - time1);
    println!("{:?}", time3 - time1);
}
