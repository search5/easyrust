use lazy_static::lazy_static;
use reqwest::Client;
use std::sync::Mutex;

struct Logger {
    logs: Mutex<Vec<Log>>,
    url: String,
    client: Client,
}

struct Log {
    message: String,
    timestamp: i64,
}

lazy_static! {
    static ref GLOBAL_LOGGER: Logger = Logger {
        logs: Mutex::new(vec![]),
        url: "https://somethingsomething.com".to_string(),
        client: Client::default()
    };
}

fn main() {
    GLOBAL_LOGGER.logs.lock().unwrap().push(Log {
        message: "Everything's going well".to_string(),
        timestamp: 1658930674
    });
}
