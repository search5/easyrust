mod print_things {
    use std::fmt::{Display, Debug};

    #[derive(Debug)]
    pub struct Billy { // Billy는 공개됩니다.
        name: String, // 그러나 name은 비공개입니다.
        pub times_to_print: u32,
    }

    impl Billy {
        pub fn new(times_to_print: u32) -> Self { // 즉, Billy를 생성하려면 사용자가
                                                  // new를 사용해야 합니다. 사용자는 times_to_print 횟수만 변경할 수 있습니다.
            Self {
                // 이름은 우리가 선택합니다. 사용자는 선택할 수 없습니다.
                name: "Billy".to_string(),
                times_to_print,
            }
        }

        pub fn print_billy(&self) { // 이 함수는 Billy를 출력합니다.
            for _ in 0..self.times_to_print {
                println!("{:?}", self.name);
            }
        }
    }

    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input)
    }
}

fn main() {
    // 이제 *를 사용해서 print_things에서 모든 것을 가져옵니다.
    use print_things::*;
    
    let my_billy = Billy::new(3);
    my_billy.print_billy();
}
