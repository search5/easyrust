struct GivesOne;

impl Iterator for GivesOne {
    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {
        Some(1)
    }
}
