#[derive(Debug)]
struct City {
    name: &'static str, // 전체 프로그램을 위해 살아 있어야 합니다.
    date_founded: u32,
}

fn main() {
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    // city_names는 전체 프로그램 안에서 살아 있지 않습니다.

    let my_city = City {
        name: &city_names[0], // ⚠ 이것은 &str이지만, &'static str은 아닙니다.
                              // city_names 내의 값에 대한 참조입니다.
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}
