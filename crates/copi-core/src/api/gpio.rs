use axum::{Json, extract::State};
use copi_protocol::Command;

use crate::{AppState, types::*};

#[axum::debug_handler]
pub async fn output_init(State(state): State<AppState>, Json(req): Json<PostGpioOutputInitReq>) {
    let cmd = Command::GpioOutputInit {
        rid: req.rid,
        pin: req.pin,
        value: req.value,
    };
    state.cmd_tx.send(cmd).ok();
}

#[axum::debug_handler]
pub async fn output_set(State(state): State<AppState>, Json(req): Json<PostGpioOutputSetReq>) {
    let cmd = Command::GpioOutputSet {
        rid: req.rid,
        pin: req.pin,
        state: req.state,
    };
    state.cmd_tx.send(cmd).ok();
}
