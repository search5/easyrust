#[derive(Debug)]
struct City<'a> { // City는 'a 수명을 가집니다.
    name: &'a str, // name에도 수명 'a 가 있습니다.
    date_founded: u32,
}

fn main() {
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}
