struct Animal { // 간단한 구조체. Animal에는 name만 있습니다.
    name: String,
}

trait Dog { // Dog 트레이트는 일부 기능을 제공합니다.
    fn bark(&self) { // 개는 짖을 수 있습니다.
        println!("Woof woof!");
    }
    fn run(&self) { // 그리고 개는 달릴 수 있습니다.
        println!("The dog is running!");
    }
}

impl Dog for Animal {
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();
}
