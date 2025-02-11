#[derive(Debug)] // 🚧
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
    can_use: bool, // 사용자가 캐릭터를 사용할 수 있는지를 설정합니다.
}

// 생략
    fn new() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
            can_use: true, // .new()는 항상 정상적인 캐릭터를 제공하므로 true입니다.
        }
    }
