use std::collections::VecDeque;

fn check_remaining(input: &VecDeque<(&str, bool)>) { // 각 항목은 (&str, bool)
    for item in input {
        if item.1 == false {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap(); // 뒤쪽에서 꺼냅니다.
    task_done.1 = true; // '완료'를 의미하는 true로 변경합니다.
    input.push_front(task_done); // 앞쪽에 밀어 넣습니다.
}

fn main() {
    let mut my_vecdeque = VecDeque::new();
    let things_to_do = vec!["send email to customer", "add new product to list", "phone Loki back"];

    for thing in things_to_do {
        my_vecdeque.push_front((thing, false));
    }
    
    done(&mut my_vecdeque);
    done(&mut my_vecdeque);
    
    check_remaining(&my_vecdeque);
    
    for task in my_vecdeque {
        print!("{task:?} ");
    }
}
