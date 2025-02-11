fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2); // vec의 길이는 2입니다.
    assert_eq!(vec[0], 1); // vec[0]은 1입니다.
    assert_eq!(vec.pop(), Some(2)); // .pop()을 사용하면 Some()을 얻습니다.
    assert_eq!(vec.len(), 1); // vec의 길이는 이제 1입니다.
    
    vec[0] = 7;
    assert_eq!(vec[0], 7); // Vec[0]은 7입니다.
    
    vec.extend([1, 2, 3].iter().copied());
    for x in &vec {
        println!("{}", x);
    }
    
    assert_eq!(vec, [7, 1, 2, 3]); // .vec은 이제 [7, 1, 2, 3]입니다.
}
