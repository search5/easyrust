fn return_two() -> i8 {
    2
}

fn return_six() -> i8 {
    4 + return_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_six() {
        assert_eq!(return_six(), 6)
    }

    #[test]
    fn it_returns_two() {
        assert_eq!(return_two(), 2);
    }
}
