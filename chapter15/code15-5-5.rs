use rayon::prelude::*; // rayon 가져오기

fn main() {
    let mut my_vec = vec![0; 200_000];
    my_vec
        .par_iter_mut() // iter_mut에 par_를 추가합니다.
        .enumerate()
        .for_each(|(index, number)| *number += index + 1);
    println!("{:?}", &my_vec[5000..5005]);
}
