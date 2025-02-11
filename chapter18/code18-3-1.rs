fn main() {
    let some_number = 200_u8;
    let other_number = 200_u8;
    
    println!("{:?}", some_number.checked_add(other_number));
    println!("{:?}", some_number.checked_add(1));
}
