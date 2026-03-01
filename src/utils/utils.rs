use env_logger::{builder, init};
use log::{info, warn, error, LevelFilter};

pub fn init_rt() {
    init_loger()
}

fn init_loger() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .init();
    info!("Logger is inited");
}