use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{Router, routing::post};
use tokio_serial::{SerialPortBuilderExt as _, SerialStream};

mod api;
mod types;

#[derive(Clone)]
pub struct AppState {
    port: Arc<Mutex<SerialStream>>,
}

pub async fn run() {
    env_logger::init();

    let mut port = tokio_serial::new("/dev/tty.usbmodem123456781", 0)
        .open_native_async()
        .unwrap();

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let state = AppState {
        port: Arc::new(Mutex::new(port)),
    };

    let app = Router::new()
        .route("/gpio/output-init", post(api::gpio::output_init))
        .route("/gpio/output-set", post(api::gpio::output_set))
        .route("/pwm/init", post(api::pwm::init))
        .route(
            "/pwm/set-duty-cycle-percent",
            post(api::pwm::set_duty_cycle_percent),
        )
        .route("/pio/load_program", post(api::pio::load_program))
        .route("/pio/sm_init", post(api::pio::sm_init))
        .route("/pio/sm_set_enabled", post(api::pio::sm_set_enabled))
        .route("/pio/sm_push", post(api::pio::sm_push))
        .route("/pio/sm_exec_instr", post(api::pio::sm_exec_instr))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8899").await.unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
