fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
    return Ok(())
    } else {
    return Err(())
    }
   }
   
   fn main() {
    if give_result(5).is_ok() {
    println!("It's okay, guys")
    } else {
    println!("It's an error, guys")
    }
   }