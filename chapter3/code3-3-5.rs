struct Colour(u8, u8, u8); // 동일한 Colour 튜플 구조체를 선언합니다.
struct SizeAndColour {
    size: u32,
    colour: Colour, // 여기에 쉼표가 없습니다.
}

fn main() {}
