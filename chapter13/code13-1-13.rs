#[derive(Default)]
enum Jobs {
    Firefighter,
    Pilot,
    // Country라는 데이터를 포함하므로 King에는 #[default]를 사용할 수 없습니다.
    // 물론 원한다면 impl Default를 사용해 King을 기본값으로 설정할 수 있습니다.
    King(Country),
    #[default]
    OfficeWorker,
}

struct Country {
    name: String,
}
