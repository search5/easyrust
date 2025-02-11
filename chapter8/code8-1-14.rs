fn main() {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);

    let mut counter = 0; // 카운트 시작
    let mut big_iter = big_vec.into_iter(); // 이터레이터를 만듭니다.
    
    loop {
        counter +=1;

        if big_iter.next() == Some(5) {
            // Some(5)를 얻을 때까지 .next()를 계속 호출합니다.
            break;
        }
    }

    println!("Final counter is: {}", counter);
}
