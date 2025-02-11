fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10; // *를 사용해서 i32 값을 변경하세요.
    println!("{}", my_number);
    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Second_number = triple_reference? {}", second_number == ***triple_reference);
}
