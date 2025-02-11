fn main() {
    let bool_vec = vec![true, false, true, false, false];

    let option_vec = bool_vec
        .iter()
        .map(|item| {
            item.then(|| { // map 안에 넣어 전달할 수 있도록 하세요.
                println!("Got a {}!", item);
                "It's true, you know"
                // true라면 Some에 들어갑니다.
                // 그렇지 않으면 None을 전달합니다.
            })
        })
        .collect::<Vec<_>>();
    
    println!("Now we have: {:?}", option_vec);
    
    // None도 출력되었습니다. 새 Vec에서 map을 필터링해 보겠습니다.
    let filtered_vec = option_vec.into_iter().filter_map(|c| c).collect::<Vec<_>>();
    
    println!("And without the Nones: {:?}", filtered_vec);
}
