#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: String,
}

#[derive(Debug)]
    struct CityData {
    names: Vec<String>,
    histories: Vec<String>,
}

fn main() {
    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000,
        // 이 문자열이 매우 긴 척합니다.
        city_history: "Calgary began as a fort called Fort Calgary that...".to_string(),
    };

    let canada_cities = CityData {
        names: vec![calgary.name], // calgary.name은 짧습니다.
        histories: vec![calgary.city_history], // 그러나 이 문자열은 매우 깁니다.
    };
    
    println!("Calgary's history is: {}", calgary.city_history); // ⚠
}
