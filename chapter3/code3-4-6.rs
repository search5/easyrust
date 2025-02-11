enum Season {
    Spring, // 이 암이 Spring(String)이거나 다른 것이라면 동작하지 않을 것입니다.
    Summer,
    Autumn,
    Winter,
}

fn main() {
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as u32);
    }
}
