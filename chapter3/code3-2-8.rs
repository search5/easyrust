fn main() {
    let children = 5;
    let married = true;
    match (children, married) {
        (children, married) if married == false =>
            println!("Not married with {} children", children),
        (children, married) if children == 0 && married == true =>
            println!("Married but no children"),
        _ => println!("Married? {}. Number of children: {}.", married, children),
    }
}
