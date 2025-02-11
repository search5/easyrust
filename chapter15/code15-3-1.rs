use std::sync::Mutex;

#[derive(Debug)]
struct Log {
    // 타임스탬프는 일반적으로 i64이지만, 여기서는 &str만 사용합니다.
    date: &'static str,
    message: String,
}

// 내부에 아무것도 없으며 할당도 하지 않으므로 static으로도 괜찮습니다.
// 그리고 Mutex이므로 내부의 내용을 변경할 수 있습니다.
static GLOBAL_LOGGER: Mutex<Vec<Log>> = Mutex::new(Vec::new());

fn do_thing(date: &'static str) {
    // 전역이므로 함수에 전달할 필요가 없습니다.
    GLOBAL_LOGGER.lock().unwrap().push(Log {
        date,
        message: "Everything's fine".to_string(),
    });
}

fn main() {
    do_thing("2022-12-12");
    do_thing("2023-05-05");
    println!("{GLOBAL_LOGGER:#?}");
}
