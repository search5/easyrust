use once_cell::sync::OnceCell;
use reqwest::Client;
use std::sync::Mutex;

#[derive(Debug)]
struct Logger {
    logs: Mutex<Vec<Log>>,
    0url: String,
    client: Client,
}

#[derive(Debug)]
struct Log {
    message: String,
    timestamp: i64,
}

static GLOBAL_LOGGER: OnceCell<Logger> = OnceCell::new();

fn fetch_url() -> String {
    // 몇 가지 작업을 수행합니다.
    "http://somethingsomething.com".to_string()
}

fn main() {
    let url = fetch_url(); // 그냥 문자열입니다.
    // 이제 설정할 차례입니다. 여기에 Logger 구조체를 밀어 넣습니다.
    GLOBAL_LOGGER.set(Logger {
        logs: Mutex::new(vec![]),
        url,
        client: Client::default(),
        }).unwrap(); // 설정이 동작하지 않을 수 있으므로(예: 두 번 호출하는 경우)
                     // 결괏값을 반환합니다. 여기서 래핑을 풉니다.
    
    // 이제 GLOBAL_LOGGER가 초기화되었습니다. 이에 대한 참조를 가져오겠습니다.
    GLOBAL_LOGGER
        .get() // .get()은 Option<&T>를 반환합니다.
        .unwrap() // 래핑을 해제합니다.
        .logs // 로그에 접근합니다.
        .lock() // Mutex이므로 잠급니다.
        .unwrap() // .lock()은 Result를 반환하므로 여기서 다시 래핑을 해제합니다.
        .push(Log { // 그리고 마지막으로 Vec에 밀어 넣을 수 있습니다.
            message: "Everything's going well".to_string(),
            timestamp: 1658930674,
        });
    
    // 완료!
    println!("{GLOBAL_LOGGER:?}");
}
