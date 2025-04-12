uniffi::include_scaffolding!("export");

use copi_core::AppState;
use log::LevelFilter;
use log::info;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static G_TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Failed to create Tokio runtime"));

enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Into<LevelFilter> for LogLevel {
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[allow(unused_variables)]
fn init_logger(level: LogLevel) {
    #[cfg(target_os = "android")]
    android_logger::init_once(android_logger::Config::default().with_max_level(level.into()));
}

fn init_usb_fd(fd: i32, interface_comm: i32, interface_data: i32) {
    let (request_tx, request_rx) = tokio::sync::mpsc::unbounded_channel();
    let (response_tx, response_rx) = tokio::sync::mpsc::unbounded_channel();

    #[cfg(not(target_os = "android"))]
    let state = AppState::new(request_tx, response_rx); // unreachable!();

    #[cfg(target_os = "android")]
    let state = AppState::new(request_tx, response_rx, &G_TOKIO_RUNTIME);

    info!("Connect to USB fd:{}", fd);
    G_TOKIO_RUNTIME.spawn(copi_core::mobile::start_usb_cdc_service(
        fd,
        interface_comm,
        interface_data,
        request_rx,
        response_tx,
    ));
    info!("Start API service");
    G_TOKIO_RUNTIME.spawn(copi_core::start_api_service(state));
}
