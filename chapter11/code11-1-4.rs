#[derive(Debug)]
struct City {
    name: String,
    years: Vec<u32>,
    populations: Vec<u32>,
}

impl City {
    fn new(name: &str, years: Vec<u32>, populations: Vec<u32>) -> Self {
        Self {
            name: name.to_string(),
            years,
            populations,
        }
    }

    fn change_city_data<F>(&mut self, mut f: F) // self를 가져오지만, f만 제네릭 F입니다. f는 클로저입니다.
    where
        F: FnMut(&mut Vec<u32>, &mut Vec<u32>), // 클로저는 연도 및 인구 데이터인 u32의 가변 벡터를 사용합니다.
    {
    f(&mut self.years, &mut self.populations)
        // 마지막으로 이것이 실제 함수입니다.
        // '이 클로저의 입력은 self.years와 self.populations'라는 의미입니다.
        // 클로저로 원하는 것은 무엇이든 할 수 있습니다.
    }
}

fn main() {
    let years = vec![
        1372, 1834, 1851, 1881, 1897, 1925, 1959, 1989, 2000, 2005, 2010, 2020,
    ];

    let populations = vec![
        3_250, 15_300, 24_000, 45_900, 58_800, 119_800, 283_071, 478_974, 400_378,
        401_694, 406_703, 437_619,
    ];

    // 이제 city를 만들 수 있습니다.
    let mut tallinn = City::new("Tallinn", years, populations);

    // 이제 클로저가 있는 .change_city_data() 메서드가 있습니다.
    // 우리는 원하는 무엇이든 할 수 있습니다.
    // 먼저 5년간의 데이터를 모아서 출력해 봅시다.
    tallinn.change_city_data(|city_years, city_populations| {
        // 입력을 원하는 대로 제공해서 호출할 수 있습니다.
        let new_vec = city_years
            .iter_mut()
            .zip(city_populations.iter_mut()) // 두 개를 함께 묶으세요.
            .take(5) // 하지만 처음 5개만 가져가세요.
            .collect::<Vec<(_, _)>>(); // 튜플 내부의 타입을 결정하라고
                                       // 러스트에 지시하세요.
        println!("{:?}", new_vec);
    });

    // 이제 2030년의 데이터를 추가하겠습니다.
    tallinn.change_city_data(|x, y| {
        // 이번에는 입력 x와 y를 호출합니다.
        x.push(2030);
        y.push(500_000);
    });

    // 더 이상 1834년의 데이터를 원하지 않습니다.
    tallinn.change_city_data(|x, y| {
        let position_option = x.iter().position(|x| *x == 1834);

        if let Some(position) = position_option {
            println!(
                "Going to delete {} at position {:?} now.",
                x[position], position
            ); // 올바른 항목을 삭제했는지 확인합니다.

            x.remove(position);
            y.remove(position);
        }
    });

    println!(
        "Years left are {:?}\nPopulations left are {:?}",
        tallinn.years, tallinn.populations
    );
}
