fn main() {
    let mut my_vec = vec![0; 200_000];

    my_vec
        .iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number += index + 1);
    println!("{:?}", &my_vec[5000..5005]);
}
