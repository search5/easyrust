#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
    can_use: bool, // 여기에 새로운 값이 들어옵니다.
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

impl Character {
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

    fn height(mut self, height: u32) -> Self {
        self.height = height;
        self.can_use = false; // 이제 사용자는 캐릭터를 사용할 수 없습니다.
        self
    }

    fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self.can_use = false;
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self.can_use = false;
        self
    }

    fn build(mut self) -> Result<Character, String> {
        if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf") {
            self.can_use = true; // 모든 것이 정상이므로 true로 설정합니다.
            Ok(self)
            // 캐릭터를 반환합니다.
        } else {
            Err("Could not create character. Characters must have:
1) Height below 200
2) Weight below 300
3) A name that is not Smurf (that is a bad word)".to_string())
        }
    }
}

fn main() {
    // 금지 단어인 "smurf"를 포함합니다.
    let character_with_smurf = Character::new().name("Lol I am Smurf!!").build();

    // 지정한 값보다 키가 큽니다.
    let character_too_tall = Character::new().height(400).build();
    
    // 지정한 값보다 체중이 무겁습니다.
    let character_too_heavy = Character::new().weight(500).build();

    let okay_character = Character::new()
        .name("Billybrobby")
        .height(180)
        .weight(100)
        .build(); // 이 캐릭터는 이름, 키, 몸무게 모두 정상입니다.
    
    // 이제 Character가 아니라 Result<Character, String>입니다.
    // Vec에 넣어 우리가 볼 수 있도록 합니다.
    let character_vec = vec![character_with_smurf, character_too_tall, character_too_heavy, okay_character];
    
    for character in character_vec {
        // OK면 캐릭터를 출력하고 Err이면 오류를 출력합니다.
        match character {
            Ok(character_info) => println!("{:?}", character_info),
            Err(err_info) => println!("{}", err_info),
        }
        
        println!(); // 빈 줄을 하나 출력합니다.
    }
}
