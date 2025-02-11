struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let kalmykia = Country {
        population: 500_000,
        capital: String::from("Elista"),
        leader_name: String::from("Batu Khasikov"),
    };
}
