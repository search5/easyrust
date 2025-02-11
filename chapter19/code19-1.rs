macro_rules! give_six {
    () => {
        6
    };
}

fn main() {
    let six = give_six!();
    println!("{}", six);
}
