fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number| {
            number *= 2;
            println!("Doubling number. Now it is {number}");
            number
        },
        "triple" => |mut number| {
            number *= 3;
            println!("Tripling number. Now it is {number}");
            number
        },
        _ => |number| {
            println!("Sorry, it's the same: {number}.");
            number
        },
    }
}

fn main() {
    let my_number = 10;
    
    // 세 개의 클로저를 만들겠습니다.
    let mut doubles = returns_a_closure("double");
    let mut triples = returns_a_closure("triple");
    let mut quadruples = returns_a_closure("quadruple");

    doubles(my_number);
    triples(my_number);
    quadruples(my_number);
}
