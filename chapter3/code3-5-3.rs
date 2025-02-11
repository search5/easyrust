fn main() {
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");
    'first_loop: loop {
        // 첫 번째 루프에 이름을 지정합니다.
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 5 {
            // 이 루프 안에서 두 번째 루프를 시작합니다.
            println!("Now entering the second loop.");
            'second_loop: loop {
                // 이제 'second_loop 안에 있습니다.
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop; // 프로그램을 종료할 수 있도록
                                                 // 'first_loop에서 벗어나세요.
                }
            }
        }
    }
}
