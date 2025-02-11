use reqwest::Client;

struct Logger {
    logs: Vec<Log>,
    url: String,
    client: Client,
}

struct Log {
    message: String,
    timestamp: i64,
}
