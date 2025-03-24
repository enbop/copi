use std::{
    io::Write,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{Json, Router, extract::State, routing::post};
use copi_protocol::Command;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio_serial::{SerialPortBuilderExt as _, SerialStream};

const MAX_USB_PACKET_SIZE: usize = 64;

#[derive(Clone)]
pub struct AppState {
    port: Arc<Mutex<SerialStream>>,
}

#[tokio::main]
async fn main() {
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
        .route("/gpio-output-init", post(post_gpio_output_init))
        .route("/gpio-output-set", post(post_gpio_output_set))
        .route("/set-pwm", post(post_set_pwm))
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

#[derive(Debug, Clone, Deserialize)]
pub struct PostGpioOutputInitReq {
    rid: u16,
    pin: u8,
    value: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostGpioOutputInitRes {
    ok: bool,
}

#[axum::debug_handler]
pub async fn post_gpio_output_init(
    State(state): State<AppState>,
    Json(req): Json<PostGpioOutputInitReq>,
) -> Json<PostGpioOutputInitRes> {
    let mut port = state.port.lock().unwrap();

    let gpio_cmd = Command::GpioOutputInit {
        rid: req.rid,
        pin: req.pin,
        value: req.value,
    };

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&gpio_cmd);
    minicbor::encode(&gpio_cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    let res = match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent GPIO command: {:?}", gpio_cmd);
            true
        }
        Err(e) => {
            log::error!("Failed to send GPIO command: {:?}", e);
            false
        }
    };

    Json(PostGpioOutputInitRes { ok: res })
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostGpioOutputSetReq {
    rid: u16,
    pin: u8,
    state: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostGpioOutputSetRes {
    ok: bool,
}

#[axum::debug_handler]
pub async fn post_gpio_output_set(
    State(state): State<AppState>,
    Json(req): Json<PostGpioOutputSetReq>,
) -> Json<PostGpioOutputSetRes> {
    let mut port = state.port.lock().unwrap();

    let gpio_cmd = Command::GpioOutputSet {
        rid: req.rid,
        pin: req.pin,
        state: req.state,
    };

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&gpio_cmd);
    minicbor::encode(&gpio_cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    let res = match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent GPIO command: {:?}", gpio_cmd);
            true
        }
        Err(e) => {
            log::error!("Failed to send GPIO command: {:?}", e);
            false
        }
    };

    Json(PostGpioOutputSetRes { ok: res })
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostSetPwmReq {
    name: u8,
    period: u32,
    #[serde(rename = "dutyCycle")]
    duty_cycle: u32,
    percent: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostSetPwmRes {
    ok: bool,
}

#[axum::debug_handler]
pub async fn post_set_pwm(
    State(state): State<AppState>,
    Json(req): Json<PostSetPwmReq>,
) -> Json<PostSetPwmRes> {
    let mut port = state.port.lock().unwrap();

    let pwm_cmd = Command::PwmSet {
        name: req.name,
        period: req.period,
        duty_cycle: req.duty_cycle,
        percent: req.percent,
    };

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&pwm_cmd);
    minicbor::encode(&pwm_cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    let res = match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent PWM command: {:?}", pwm_cmd);
            true
        }
        Err(e) => {
            log::error!("Failed to send PWM command: {:?}", e);
            false
        }
    };

    Json(PostSetPwmRes { ok: res })
}
