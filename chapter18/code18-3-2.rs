use std::ops::Add; // Add를 먼저 추가합니다.

// PartialEq는 아마도 여기서 가장 중요한 부분일 것입니다.
// 우리는 숫자를 비교하고 싶습니다.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // 다음을 ‘연관 타입’, 즉, ‘함께 사용되는 타입’이라고 합니다.
    // 이 경우에는 다른 Point에 불과합니다.
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
