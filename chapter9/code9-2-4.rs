#[derive(Debug)]
struct City {
    name: &str, // ⚠
    date_founded: u32,
}

fn main() {
    let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };
}
