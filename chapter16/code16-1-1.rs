use rand::seq::SliceRandom; // slices에 .choose 메서드를 사용하므로 추가했습니다.

fn main() {
    let my_letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    let mut rng = rand::thread_rng();
    for _ in 0..6 {
        print!("{} ", my_letters.choose(&mut rng).unwrap());
    }
}
