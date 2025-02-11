struct GivesOne;

impl Iterator for GivesOne {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        Some(1)
    }
}

fn main() {
    let five_ones: Vec<i32> = GivesOne.into_iter().take(5).collect();
    println!("{five_ones:?}");
}
