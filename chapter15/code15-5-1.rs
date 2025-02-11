use rand;
// 이는 전체 rand 크레이트를 의미합니다.
// 컴퓨터에서는 이렇게 바로 사용할 수 없으며
// 먼저 Cargo.toml 파일에 작성해야 합니다.

fn main() {
    for _ in 0..5 {
        let random_u16 = rand::random::<u16>();
        print!("{random_u16} ");
    }
}
