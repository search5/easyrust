enum TimeOfDay {
    Dawn,
    Day,
    Sunset,
    Night,
}

fn make_fear_closure(input: TimeOfDay) -> impl FnMut(&mut f64) {
    // 이 함수는 TimeOfDay를 가져가서 클로저를 반환합니다.
    // impl FnMut(&mut f64)를 사용해 값을 변경해야 한다고 말합니다.

    match input {
        TimeOfDay::Dawn => |x: &mut f64| {
            // 이것은 나중에 제공하는 character_fear 변수입니다.
            println!(
                "The morning sun has vanquished the horrible night. You no longer feel afraid."
            );
            *x *= 0.5;
            println!("Your fear is now {x}");
        },
        TimeOfDay::Day => |x: &mut f64| {
            println!("What a nice day. Maybe put your feet up and rest a bit.");
            *x *= 0.2;
            println!("Your fear is now {x}");
        },
        TimeOfDay::Sunset => |x: &mut f64| {
            println!("The sun is almost down! This is no good.");
            *x *= 1.4;
            println!("Your fear is now {x}");
        },
        TimeOfDay::Night => |x: &mut f64| {
            println!("What a horrible night to have a curse.");
            *x *= 5.0;
            println!("Your fear is now {x}");
        },
    }
}

fn main() {
    use TimeOfDay::*;

    let mut character_fear = 10.0; // 10으로 Simon을 시작합니다.

    // Simon의 두려움을 바꾸고 싶을 때마다 호출하기 위해
    // 여기에 4개의 클로저를 만듭니다.
    let mut change_for_daytime = make_fear_closure(Day);
    let mut change_for_sunset = make_fear_closure(Sunset);
    let mut change_for_night = make_fear_closure(Night);
    let mut change_for_morning = make_fear_closure(Dawn);
    
    // Simon의 두려움에 대한 클로저를 호출합니다.
    // 클로저는 메시지를 주고 두려움 수치를 변경합니다.
    // 실생활에서는 Character 구조체가 있고 구조체를 직접 사용하는 대신
    // 대신 character_fear.daytime() 같은 메서드로 사용합니다.
    change_for_daytime(&mut character_fear);
    change_for_sunset(&mut character_fear);
    change_for_night(&mut character_fear);
    change_for_morning(&mut character_fear);
}
