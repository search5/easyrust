const NUMBER: u8 = give_eight();

const fn give_eight() -> u8 {
    8
}

fn main() {
    let mut my_vec = Vec::new();
    my_vec.push(give_eight());
}
