fn main() {
    let mut join_vec = vec![];

    for num in 0..10 {
        join_vec.push(std::thread::spawn(move || { // num을 안으로 이동시킵니다.
            println!("I am printing something: {num}");
        }));
    } // 10개의 스레드가 시작되어 현재 동시에 작동 중입니다.

    for handle in join_vec {
        // 모든 스레드에 각각 .join()을 호출하여 스레드가 완료될 때까지 대기합니다.
        handle.join().unwrap();
    }
}
