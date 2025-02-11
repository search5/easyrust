use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}

fn main() {}
