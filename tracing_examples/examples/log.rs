fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    log::debug!("this is a log line");
    tracing::debug!("this is a tracing line");
}
