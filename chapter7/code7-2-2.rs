fn main() {
    let my_vec = vec!['a', 'b', '거', '柳']; // 일반 Vec

    let mut my_vec_iter = my_vec.iter();
    // 현재 이터레이터 타입이지만 아직 호출하지 않았습니다.

    assert_eq!(my_vec_iter.next(), Some(&'a')); // .next()로 첫 번째 항목 호출
    assert_eq!(my_vec_iter.next(), Some(&'b')); // 다음 항목 호출
    assert_eq!(my_vec_iter.next(), Some(&'거')); // 다시 한번 호출
    assert_eq!(my_vec_iter.next(), Some(&'柳')); // 다시 한번 호출
    assert_eq!(my_vec_iter.next(), None); // 아무것도 남지 않았음: None
    assert_eq!(my_vec_iter.next(), None); // .next()를 계속 호출할 수 있지만
                                          // 항상 None입니다.
}
