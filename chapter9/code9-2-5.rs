#[derive(Debug)]
struct City {
    name: &'static str, // &str을 &'static str로 변경
    date_founded: u32,
}

fn main() {
    let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}
