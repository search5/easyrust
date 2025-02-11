use rand::{thread_rng, Rng}; // 귀찮다면 rand::*;를 사용해도 됩니다.

fn main() {
    let mut number_maker = thread_rng();
    for _ in 0..5 {
        print!("{} ", number_maker.gen_range(1..11));
    }
}
