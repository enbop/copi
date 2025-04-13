use axum::{Json, extract::State, http::StatusCode};
use copi_protocol::HostMessage;

use crate::{AppState, process_common, types::*};

#[axum::debug_handler]
pub async fn init(
    State(state): State<AppState>,
    Json(req): Json<PostPwmInitReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let msg = HostMessage::PwmInit {
        slice: req.slice,
        a: req.a,
        b: req.b,
        divider: req.divider,
        compare_a: req.compare_a,
        compare_b: req.compare_b,
        top: req.top,
    };
    process_common!(state, req, msg)
}

#[axum::debug_handler]
pub async fn set_duty_cycle_percent(
    State(state): State<AppState>,
    Json(req): Json<PostPwmSetDutyCyclePercentReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let msg = HostMessage::PwmSetDutyCyclePercent {
        pin: req.pin,
        percent: req.percent,
    };
    process_common!(state, req, msg)
}
