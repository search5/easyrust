use std::fmt;

struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old.", self.name, self.age)
    }
}

fn print_cats(pet: String) {
    println!("{pet}");
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    print_cats(mr_mantle.to_string()); // 여기에서 mr_mantle을 String으로 바꿉니다.
    println!("Mr. Mantle's String is {} letters long.", mr_mantle.to_string().chars().count()); // 여기에서 String을 char로 바꾸어 개수를 셉니다.
}
