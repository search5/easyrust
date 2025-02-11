struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");
    let kalmykia = Country {
        population: population,
        capital: capital,
        leader_name: leader_name,
    };
}
