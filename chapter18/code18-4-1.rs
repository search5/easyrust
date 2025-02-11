fn four_operations(input: f64) {
    println!(
"For the number {}:
floor: {}
ceiling: {}
rounded: {}
truncated: {}\n",
    input,
    input.floor(),
    input.ceil(),
    input.round(),
    input.trunc()
    );
}

fn main() {
    four_operations(9.1);
    four_operations(100.7);
    four_operations(-1.1);
    four_operations(-19.9);
}
