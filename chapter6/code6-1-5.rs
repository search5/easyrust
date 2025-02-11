struct Animal { // 간단한 구조체. Animal(동물)에는 name만 있습니다.
    name: String,
}

trait Dog {
    // Dog(개) 트레이트는 몇 가지 기능을 제공합니다.
    fn bark(&self) {
        // 개는 짖을 수 있습니다.
        println!("Woof woof!");
    }

    fn run(&self) {
        // 그리고 개는 달릴 수 있습니다.
        println!("The dog is running!");
    }
}

impl Dog for Animal {} // 이제 Animal은 Dog 트레이트를 가집니다.

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark(); // 이제 Animal은 bark()를 사용할 수 있습니다.
    rover.run(); // 그리고 run()을 사용할 수 있습니다.
}
