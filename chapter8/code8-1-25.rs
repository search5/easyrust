fn main() {
    let locations = vec![
        ("Nevis", 25),
        ("Taber", 8428),
        ("Markerville", 45),
        ("Cardston", 3585),
    ];

    let mut location_iter = locations.iter().peekable();

    while location_iter.peek().is_some() {
        match location_iter.peek() {
            Some((name, number)) if *number < 100 => {
                // .peek()은 참조를 제공하므로 *가 필요합니다.
                println!("Found a hamlet: {name} with {number} people")
            }
            
            Some((name, number)) => println!("Found a town: {name} with {number} people"),
            None => break,
        }

        location_iter.next();
    }
}
