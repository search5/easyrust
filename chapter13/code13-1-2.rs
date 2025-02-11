struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain
}

impl Character {
    fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
        Self {
            name,
            age,
            height,
            weight,
            lifestate: if alive { LifeState::Alive } else { LifeState::Dead },
        }
    }
}

fn main() {
    let character_1 = Character::new("Billy".to_string(), 15, 170, 70, true);
}
