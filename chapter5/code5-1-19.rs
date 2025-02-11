use std::collections::VecDeque;

fn main() {
    let mut my_vec = VecDeque::from(vec![0; 600000]);
    
    for i in 0..600000 {
        my_vec.pop_front(); // pop_front는 .pop과 비슷하지만 앞에서 꺼내는 용도입니다.
    }
}
