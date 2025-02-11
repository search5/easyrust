fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    let some_vec = vec![9, 8, 10];
    do_something(|| {
        some_vec
            .into_iter()
            .for_each(|x| println!("The number is: {x}"));
    })
}
