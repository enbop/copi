use std::net::SocketAddr;

use axum::{Router, routing::post};
use copi_protocol::Command;
use tokio::{
    io::AsyncWriteExt,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

mod api;
// #[cfg(target_os = "android")]
pub mod mobile;
mod types;

pub const MAX_USB_PACKET_SIZE: usize = 64;

#[derive(Clone)]
pub struct AppState {
    cmd_tx: UnboundedSender<Command>,
}

impl AppState {
    pub fn new(cmd_tx: UnboundedSender<Command>) -> Self {
        AppState { cmd_tx }
    }
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub fn open_copi_serial() -> tokio_serial::SerialStream {
    use tokio_serial::SerialPortBuilderExt as _;

    let Some(device) = serialport::available_ports()
        .unwrap()
        .into_iter()
        .find(|s| match &s.port_type {
            serialport::SerialPortType::UsbPort(info) => info.vid == 49374 && info.pid == 51966,
            _ => false,
        })
    else {
        log::warn!("Device not found");
        std::process::exit(1);
    };

    log::info!("Found device: {:?}", device.port_name);

    tokio_serial::new(device.port_name, 0)
        .open_native_async()
        .unwrap()
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub async fn start_usb_cdc_service(
    mut port: tokio_serial::SerialStream,
    mut cmd_rx: UnboundedReceiver<Command>,
) {
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
