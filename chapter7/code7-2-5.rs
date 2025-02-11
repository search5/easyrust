// Some과 None을 번갈아 반복하는 이터레이터
struct Alternate {
    state: i32,
}

impl Iterator for Alternate {
    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {
        let val = self.state;
        self.state = self.state + 1;

        // 짝수이면 Some(i32), 아니면 None
        if val % 2 == 0 {
            Some(val)
        } else {
            None
        }
    }
}

fn main() {}
