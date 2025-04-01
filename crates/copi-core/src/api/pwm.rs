use axum::{Json, extract::State};
use copi_protocol::Command;

use crate::{AppState, types::*};

#[axum::debug_handler]
pub async fn init(State(state): State<AppState>, Json(req): Json<PostPwmInitReq>) {
    let cmd = Command::PwmInit {
        rid: 1,
        slice: req.slice,
        a: req.a,
        b: req.b,
        divider: req.divider,
        compare_a: req.compare_a,
        compare_b: req.compare_b,
        top: req.top,
    };
    state.cmd_tx.send(cmd).ok();
}

#[axum::debug_handler]
pub async fn set_duty_cycle_percent(
    State(state): State<AppState>,
    Json(req): Json<PostPwmSetDutyCyclePercentReq>,
) {
    let cmd = Command::PwmSetDutyCyclePercent {
        rid: 1,
        pin: req.pin,
        percent: req.percent,
    };
    state.cmd_tx.send(cmd).ok();
}
