#[derive(Debug)] // Debug는 다른 구조체와 마찬가지로 모든 크기의 배열에서 동작합니다!
struct Buffers<T, const N: usize> {
    array_one: [T; N],
    array_two: [T; N],
}

fn main() {
    let buffer_1 = Buffers {
        array_one: [0u8; 3],
        array_two: [0; 3],
    };

    let buffer_2 = Buffers {
        array_one: [0i32; 4],
        array_two: [10; 4],
    };
    
    println!("{buffer_1:#?}, {buffer_2:#?}");
}
