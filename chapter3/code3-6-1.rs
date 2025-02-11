#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new() -> Self {
        // Self는 Animal(동물)입니다.
        // Self 대신 Animal을 쓸 수도 있습니다.
        Self {
            // Animal::new()를 쓸 때 항상 10살인 고양이를 얻습니다.
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }
    fn change_to_dog(&mut self) {
        // 우리는 Animal 안에 있으므로 &mut self는 &mut Animal을 의미합니다.
        // .change_to_dog()를 호출해 &mut self를 사용해 고양이를 개로 변경합니다.
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog! Now it's {self:?}");
    }

    fn change_to_cat(&mut self) {
        // .change_to_cat()을 사용해 개를 고양이로 변경합니다.
        // &mut self의 animal_type을 변경합니다.
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat! Now it's {self:?}");
    }
    fn check_type(&self) {
        // self를 읽고 싶습니다.
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
}

fn main() {
    let mut new_animal = Animal::new(); // 새로운 동물을 생성하는 관련 기능입니다.
    // 10살인 고양이를 생성합니다.
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
}
