#[derive(Debug)]
pub struct Character {
    name: String,
    age: u8,
}

// name과 age의 기본값에는 문제가 없습니다.
impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
        }
    }
}

#[derive(Debug)]
pub struct CharacterBuilder {
    pub name: String,
    pub age: u8,
}

impl CharacterBuilder {
    fn new(name: String, age: u8) -> Self {
        // CharacterBuilder를 반환하므로 문제가 없습니다.
        Self { name, age }
    }

    // 적절한 오류 타입이어야만 하지만 간단하게 유지하겠습니다.
    fn try_build(self) -> Result<Character, &'static str> {
        if !self.name.to_lowercase().contains("smurf") {
            Ok(Character {
                name: self.name,
                age: self.age,
            })
        } else {
            Err("Can't make a character with the word 'smurf' inside it!")
        }
    }
}

fn do_something_with_character(character: &Character) {}

fn main() {
    let default_character = Character::default();
    do_something_with_character(&default_character);

    let second_character = CharacterBuilder::new("Bobby".to_string(), 27).try_build().unwrap();
    do_something_with_character(&second_character);

    let bad_character = CharacterBuilder::new("Smurfysmurf".to_string(), 40).try_build();
    
    println!("{default_character:?}");
    println!("{second_character:?}");
}
