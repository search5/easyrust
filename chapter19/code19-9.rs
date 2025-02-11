macro_rules! print_anything {
    ($($input1:tt),*) => {
        let output = stringify!($($input1),*);
        println!("{}", output);
    };
}

fn main() {
    print_anything!(ththdoetd, rcofe);
    print_anything!();
    print_anything!(87575oehq75onth, ntohe, 987987o, 097);
}
