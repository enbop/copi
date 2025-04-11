use axum::{Json, extract::State, http::StatusCode};
use copi_protocol::HostMessage;

use crate::{AppState, process_common, types::*};

#[axum::debug_handler]
pub async fn output_init(
    State(mut state): State<AppState>,
    Json(req): Json<PostGpioOutputInitReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let msg = HostMessage::GpioOutputInit {
        pin: req.pin,
        value: req.value,
    };
    process_common!(state, req, msg)
}

#[axum::debug_handler]
pub async fn output_set(
    State(mut state): State<AppState>,
    Json(req): Json<PostGpioOutputSetReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let msg = HostMessage::GpioOutputSet {
        pin: req.pin,
        state: req.state,
    };
    process_common!(state, req, msg)
}
