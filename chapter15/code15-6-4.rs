lazy_static! {
    static ref GLOBAL_LOGGER: Logger = Logger {
        logs: Mutex::new(vec![]),
        url: "https://somethingsomething.com".to_string(),
        client: Client::default()
    };
}
