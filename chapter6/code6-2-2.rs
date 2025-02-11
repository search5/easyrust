#[derive(Debug)] // 따라서 City를 출력할 수 있습니다.
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self { // 그냥 new 함수입니다.
        Self {
            name: name.to_string(),
            population,
        }
    }
}

#[derive(Debug)] // Country도 출력해야 합니다.
struct Country {
    cities: Vec<City>, // City는 여기에 들어갑니다.
}

impl From<Vec<City>> for Country {
    // 참고: From<City> 형태뿐만 아니라 From<Vec<City>>를 쓸 수도 있습니다.
    // 따라서 우리가 만들지 않은 타입에서도 From을 구현할 수 있습니다.
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        // Country에 있는 도시를 출력하는 기능
        for city in &self.cities {
            // Vec<City>가 Copy를 구현하지 않아서 &가 필요합니다.
            println!("{:?} has a population of {:?}.", city.name, city.population);
        }
    }
}

fn main() {
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku]; // 이것은 Vec<City>입니다.
    let finland = Country::from(finland_cities); // 이제 From을 사용할 수 있습니다.
    
    finland.print_cities();
}
