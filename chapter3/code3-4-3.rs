enum ThingsInTheSky {
    Sun(String), // 이제 각 배리언트에는 문자열이 있습니다.
    Stars(String),
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")),
        // 여기에 문자열을 씁니다.
        _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{}", description),
        // 데이터의 이름을 지정해 match에서 바로 사용할 수 있습니다.
        ThingsInTheSky::Stars(n) => println!("{}", n),
        // 또는 데이터의 이름을 n으로 지정할 수도 있습니다.
        // 이 외 다른 어떤 이름도 사용할 수 있으므로 이름은 중요하지 않습니다.
    }
}

fn main() {
    let time = 8; // 8시입니다.
    let skystate = create_skystate(time); // create_skystate는 ThingsInTheSky를
    // 반환합니다.
    check_skystate(&skystate); // 변수 skystate를 읽을 수 있도록 참조를 제공합니다.
}
