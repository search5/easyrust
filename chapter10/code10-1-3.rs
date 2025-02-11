type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>>;

fn returns<'a>(input: &'a Vec<char>) -> SkipFourTakeFive {
    input.iter().skip(4).take(5)
}

fn main() {}
