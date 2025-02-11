pub mod something {
    pub mod third_mod {
        pub fn print_a_country(input: &mut Vec<&str>) {
            println!(
                "The last country is {} inside the module {}",
                input.pop().unwrap(),
                module_path!()
            );
        }
    }
}

fn main() {
    use something::third_mod::*;
    let mut country_vec = vec!["Portugal", "Czechia", "Finland"];

    // 몇 가지 작업을 수행합니다.
    println!("Hello from file {}", file!());
    
    // 몇 가지 작업을 수행합니다.
    println!(
        "On line {} we got the country {}",
        line!(),
        country_vec.pop().unwrap()
    );

    // 더 많은 작업을 수행합니다.
    println!(
        "The next country is {} on line {} and column {}.",
        country_vec.pop().unwrap(),
        line!(),
        column!(),
    );
    
    // 더 많은 코드가 여기에 있다고 가정합니다.
    print_a_country(&mut country_vec);
}
