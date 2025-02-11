// 두 가지 선택 항목으로 enum 생성
enum ThingsInTheSky {
    Sun,
    Stars,
}

// 이 함수로 i32 타입을 사용해 ThingsInTheSky를 생성할 수 있습니다.
fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun, // 6시에서 18시 사이에 태양을 볼 수 있습니다.
        _ => ThingsInTheSky::Stars, // 그렇지 않으면 별을 볼 수 있습니다.
    }
}

// 이 함수를 사용해 ThingsInTheSky의 두 가지 선택 항목과 일치시킬 수 있습니다.
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!"),
    }
}

fn main() {
    let time = 8; // 8시입니다.
    let skystate = create_skystate(time); // create_skystate는 ThingsInTheSky를
    // 반환합니다.
    check_skystate(&skystate); // 변수 skystate를 읽을 수 있도록 참조를 제공합니다.
}
