enum MapDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn main() {}

fn give_direction(direction: &MapDirection) {
    use MapDirection::*; // MapDirection에서 모든 항목 가져오기
    let m = "You are heading";

    match direction {
        North => println!("{} north.", m),
        NorthEast => println!("{} northeast.", m),
        // 이제 좀 더 나아졌습니다.
        // ⚠
    }
}
