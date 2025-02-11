use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();
    
    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }
    
    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd"));
}
