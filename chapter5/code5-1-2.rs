use std::collections::BTreeMap; // HashMap을 BTreeMap으로 변경하기만 하면 됩니다.

struct City {
    name: String,
    population: BTreeMap<u32, u32>, // HashMap을 BTreeMap으로 변경하기만 하면 됩니다.
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(), // HashMap을 BTreeMap으로 변경하기만 하면 됩니다.
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);
    
    for (year, population) in tallinn.population {
        println!("In the year {} the city of {} had a population of {}.",
                 year, tallinn.name, population
        );
    }
}
