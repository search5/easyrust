use std::ops::Deref;

// 다른 모든 코드는 enum Alignment 이후까지 동일합니다.
struct Character {
    name: String,
    strength: u8,
    dexterity: u8,
    health: u8,
    intelligence: u8,
    wisdom: u8,
    charm: u8,
    hit_points: i8,
    alignment: Alignment,
}

impl Character {
    fn new(
        name: String,
        strength: u8,
        dexterity: u8,
        health: u8,
        intelligence: u8,
        wisdom: u8,
        charm: u8,
        hit_points: i8,
        alignment: Alignment,
    ) -> Self {
        Self {
            name,
            strength,
            dexterity,
            health,
            intelligence,
            wisdom,
            charm,
            hit_points,
            alignment,
        }
    }
}

enum Alignment {
    Good,
    Neutral,
    Evil,
}

impl Deref for Character {
    // 캐릭터에 대한 impl Deref입니다. 이제 원하는 정수 연산을 할 수 있습니다!
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.hit_points
    }
}

fn main() {
    let billy = Character::new("Billy".to_string(), 9, 8, 7, 10, 19, 19, 5, Alignment::Good); // 두 개의 캐릭터를 생성합니다.
    let brandy = Character::new("Brandy".to_string(), 9, 8, 7, 10, 19, 19, 5, Alignment::Good);

    let mut hit_points_vec = vec![]; // 여기에 hit 포인트 데이터를 넣으세요.
    hit_points_vec.push(*billy); // *billy를 넣나요?
    hit_points_vec.push(*brandy); // *brandy를 넣나요?
    
    println!("{:?}", hit_points_vec);
}
