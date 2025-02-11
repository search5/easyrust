use std::collections::HashMap;
// 매번 std::collections::HashMap을 입력하지 않아도
// HashMap을 사용할 수 있도록 해 줍니다.

struct City {
    name: String,
    population: HashMap<u32, u32>, // 해당 연도와 해당 연도의 인구가 표시됩니다.
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(), // 지금까지 HashMap은 비어 있습니다.
    };
    
    tallinn.population.insert(1372, 3_250); // 세 개의 날짜를 삽입합니다.
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        // HashMap은 HashMap<u32, u32>이므로 매번 두 개의 항목을 반환합니다.
        println!(
            "In the year {} the city of {} had a population of {}.",
            year,
            tallinn.name,
            population
        );
    }
}
