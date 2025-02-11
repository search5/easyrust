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

fn give_direction(direction: &MapDirection) {
    match direction {
        MapDirection::North => println!("You are heading north."),
        MapDirection::NorthEast => println!("You are heading northeast."),
        // 입력할 것이 훨씬 더 많이 남아 있습니다.
        // ⚠ 가능한 모든 배리언트를 쓰지 않았습니다.
    }
}

fn main() {}
