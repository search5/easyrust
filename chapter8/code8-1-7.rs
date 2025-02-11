fn main() {
    let new_vec = vec!["8", "9", "Hi", "Ninetyniney"]; // 네 개의 &str이 있는 vec
    let mut empty_vec = vec![]; // 결과가 여기에 들어갑니다.

    for index in 0..5 {
        empty_vec.push(
            new_vec
                .get(index)
                .and_then(|number| number.parse::<i32>().ok())
                .and_then(|number| f64::try_from(number).ok()),
        );
    }
    
    println!("{:?}", empty_vec);
}
