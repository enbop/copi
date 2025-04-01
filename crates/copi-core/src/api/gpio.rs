use axum::{Json, extract::State};
use copi_protocol::Command;

use crate::{AppState, types::*};

use super::send_command;

#[axum::debug_handler]
pub async fn output_init(State(state): State<AppState>, Json(req): Json<PostGpioOutputInitReq>) {
    let mut port = state.port.lock().unwrap();

    let gpio_cmd = Command::GpioOutputInit {
        rid: req.rid,
        pin: req.pin,
        value: req.value,
    };
    send_command(&mut port, gpio_cmd);
}

#[axum::debug_handler]
pub async fn output_set(State(state): State<AppState>, Json(req): Json<PostGpioOutputSetReq>) {
    let mut port = state.port.lock().unwrap();

    let gpio_cmd = Command::GpioOutputSet {
        rid: req.rid,
        pin: req.pin,
        state: req.state,
    };
    send_command(&mut port, gpio_cmd);
}
