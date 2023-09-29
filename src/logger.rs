pub fn init_logger(log_level: &Option<String>) {
    let mut level_filter = match std::env::var("RUST_LOG") {
        Ok(level) => match level.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        },
        Err(_) => log::LevelFilter::Info,
    };

    if let Some(level) = log_level {
        level_filter = match level.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        };
    }

    pretty_env_logger::formatted_timed_builder()
        .filter_level(level_filter)
        .filter_module("tract_core", log::LevelFilter::Warn)
        .filter_module("tract_hir", log::LevelFilter::Warn)
        .init();
}
