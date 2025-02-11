fn return_two() -> i8 {
    2
}

#[test]
fn it_returns_two() {
    assert_eq!(return_two(), 2);
}

fn return_six() -> i8 {
    4 + return_two()
}

#[test]
fn it_returns_six() {
    assert_eq!(return_six(), 6)
}
