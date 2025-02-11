// ⚠
struct HoldsANumber(u8);

fn main() {
    let my_number = HoldsANumber(20);
    println!("{}", *my_number + 20);
}
