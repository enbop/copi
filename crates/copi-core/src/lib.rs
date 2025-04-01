use std::net::SocketAddr;

use axum::{Router, routing::post};
use copi_protocol::Command;
use tokio::{
    io::AsyncWriteExt,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

mod api;
mod types;

const MAX_USB_PACKET_SIZE: usize = 64;

#[derive(Clone)]
pub struct AppState {
    cmd_tx: UnboundedSender<Command>,
}

impl AppState {
    pub fn new(cmd_tx: UnboundedSender<Command>) -> Self {
        AppState { cmd_tx }
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn start_usb_cdc_service(mut cmd_rx: UnboundedReceiver<Command>) {
    use tokio_serial::SerialPortBuilderExt as _;

    let mut port = tokio_serial::new("/dev/tty.usbmodem123456781", 0)
        .open_native_async()
        .unwrap();

    tokio::spawn(async move {
        let mut buf = [0u8; MAX_USB_PACKET_SIZE];
        loop {
            match cmd_rx.recv().await {
                Some(cmd) => {
                    let len = minicbor::len(&cmd);
                    minicbor::encode(&cmd, buf.as_mut()).unwrap();

                    match port.write_all(&buf[..len]).await {
                        Ok(_) => {
                            log::info!("Sent command: {:?}", cmd);
                        }
                        Err(e) => {
                            log::error!("Failed to send command: {:?}", e);
                        }
                    }
                }
                None => {
                    log::warn!("Command receiver closed");
                    break;
                }
            }
        }
    });
}

pub async fn start_api_service(state: AppState) {
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
