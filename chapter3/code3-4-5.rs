enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*; // Mood에서 모든 것을 가져왔습니다.
    // 이제 Happy, Sleepy 등을 바로 쓸 수 있습니다.
    let happiness_level = match mood {
        Happy => 10, // 더 이상 Mood::를 먼저 쓸 필요가 없습니다.
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };
    happiness_level
}

fn main() {
    let my_mood = Mood::Happy;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);
}
