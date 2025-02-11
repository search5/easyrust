use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: Rc<String>, // Rc 내부의 String
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>, // Rc들의 내부에 있는 String의 Vec
}

fn main() {
    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000,
        // 이 문자열이 매우 긴 척합니다.
        city_history: Rc::new("Calgary began as a fort called Fort Calgary \
        that...".to_string()), // Rc::new()를 사용해 Rc를 만듭니다.
    };

    let canada_cities = CityData {
        names: vec![calgary.name],
        histories: vec![calgary.city_history.clone()],
        // .clone()은 카운트를 증가시킵니다.
    };

    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));

    let new_owner = Rc::clone(&calgary.city_history);
}
