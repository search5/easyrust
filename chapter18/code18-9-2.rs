use std::{mem, fmt};

struct Ring { // 『반지의 제왕』의 반지를 만듭니다.
    owner: String,  // 현재 소유자
    former_owner: String, // 기존 소유자
    seeker: String, // 반지를 원하는 자
}

impl Ring {
    fn new(owner: &str, former_owner: &str, seeker: &str) -> Self {
        Self {
            owner: owner.to_string(),
            former_owner: former_owner.to_string(),
            seeker: seeker.to_string(),
        }
    }
}

impl fmt::Display for Ring { // 누가 가지고 있고 누가 원하는지 표시하는 디스플레이
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} has the ring, {} used to have it, and {} wants it",
            self.owner, self.former_owner, self.seeker)
    }
}

fn main() {
    let mut one_ring = Ring::new("Frodo", "Gollum", "Sauron");
    println!("{}", one_ring);
    
    mem::swap(&mut one_ring.owner, &mut one_ring.former_owner);
    
    // 골룸이 반지를 잠시 되찾았습니다.
    println!("{}", one_ring);
}
