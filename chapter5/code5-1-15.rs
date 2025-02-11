use std::collections::BinaryHeap;

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    // 이 함수는 BinaryHeap의 나머지를 보여 줍니다. 실제로 이터레이터는
    // 함수보다 빠릅니다. 이터레이터는 나중에 배울 것입니다.
    let mut remainder_vec = vec![];

    for number in input {
        remainder_vec.push(*number);
    }

    remainder_vec
}

fn main() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30]; // 숫자는 순서대로 되어 있습니다.

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }
    
    while let Some(number) = my_heap.pop() {
        // .pop()은 힙에서 숫자를 꺼내 Some(숫자)을 반환하고,
        // 비어 있으면 None을 반환합니다.
        println!("Popped off {number}. Remaining numbers are: {:?}", show_remainder(&my_heap));
    }
}
