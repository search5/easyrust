async fn async_give_8() -> u8 {
    8
}

fn main() {
    let y = async_give_8(); // async_give_8의 결과를 가져옵니다.
    y.thoethoe(); // 존재하지 않는 메서드를 사용해 오류를 확인합니다.
}
