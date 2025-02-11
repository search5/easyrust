fn prints_three_things(vector: Vec<i32>) {
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn main() {
    let my_vec = vec![8, 9, 10];
    prints_three_things(my_vec);
}
