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

static GLOBAL_LOGGER: Logger = Logger {
    logs: Mutex::new(vec![]),
    url: "https://somethingsomething.com".to_string(),
    client: Client::default()
};

fn main() {
}
