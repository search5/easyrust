use std::fmt;
use std::ops::Add;

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32, // 경제 규모를 나타내는 지표
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp,
        }
    }
}

impl Add for Country {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            name: format!("{} and {}", self.name, other.name), // 이름을 함께 담고,
            population: self.population + other.population, // 인구를 합산하며,
            gdp: self.gdp + other.gdp, // GDP도 합산합니다.
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // 그러면 {}만 사용해 모두 출력할 수 있습니다.
            "In {} are {} people and a GDP of ${}",
            self.name, self.population, self.gdp
        )
    }
}

fn main() {
    let nauru = Country::new("Nauru", 10_670, 160_000_000);
    let vanuatu = Country::new("Vanuatu", 307_815, 820_000_000);
    let micronesia = Country::new("Micronesia", 104_468, 367_000_000);

    // Country에 이름에 문자열 대신 &str을 지정할 수도 있었습니다.
    // 하지만 모든 곳에 수명을 사용해야 하고
    // 간단한 예제에서는 너무 많은 양이 될 것입니다.
    // println!을 호출할 때 그냥 복제하는 편이 더 낫습니다.
    println!("{}", nauru.clone());
    println!("{}", nauru.clone() + vanuatu.clone());
    println!("{}", nauru + vanuatu + micronesia);
}
