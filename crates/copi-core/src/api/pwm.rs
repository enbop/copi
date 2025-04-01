use axum::{Json, extract::State};
use copi_protocol::Command;

use crate::{AppState, types::*};

use super::send_command;

#[axum::debug_handler]
pub async fn init(State(state): State<AppState>, Json(req): Json<PostPwmInitReq>) {
    let mut port = state.port.lock().unwrap();

    let pwm_cmd = Command::PwmInit {
        rid: 1,
        slice: req.slice,
        a: req.a,
        b: req.b,
        divider: req.divider,
        compare_a: req.compare_a,
        compare_b: req.compare_b,
        top: req.top,
    };
    send_command(&mut port, pwm_cmd);
}

#[axum::debug_handler]
pub async fn set_duty_cycle_percent(
    State(state): State<AppState>,
    Json(req): Json<PostPwmSetDutyCyclePercentReq>,
) {
    let mut port = state.port.lock().unwrap();

    let pwm_cmd = Command::PwmSetDutyCyclePercent {
        rid: 1,
        pin: req.pin,
        percent: req.percent,
    };
    send_command(&mut port, pwm_cmd);
}
