struct Animal {
    name: String,
}

trait Dog {
    fn bark(&self); // bark()는 &self가 필요하며 아무것도 반환하지 않는다고 합니다.
    fn run(&self); // run()은 &self가 필요하고 아무것도 반환하지 않는다고 합니다.
                   // 이제 직접 만들어야 합니다.
}

impl Dog for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
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
