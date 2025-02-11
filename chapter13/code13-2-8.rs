use std::ops::{Deref, DerefMut};

struct HoldsANumber(u8);

impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsANumber {
    // Target = u8 타입이 필요하지 않습니다.
    // Deref 덕분에 이미 알고 있기 때문입니다.
    fn deref_mut(&mut self) -> &mut Self::Target {
        // 모든 곳에 mut를 붙인 것을 제외하면 나머지는 동일합니다.
        &mut self.0
    }
}

fn main() {
    let mut my_number = HoldsANumber(20);
    *my_number = 30; // DerefMut를 사용하면 이 작업을 수행할 수 있습니다.
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();
}
