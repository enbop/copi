use crate::AppState;
use crate::generated::{RequestBody, ResponseBody};
use axum::{Json, extract::State, http::StatusCode};

// TODO: Support both protobuf and JSON body
// TODO: Uncomment and implement these modules as needed
// pub mod gpio;
// pub mod pio;
pub mod playground;
// pub mod pwm;

#[axum::debug_handler]
pub async fn query(
    State(state): State<AppState>,
    Json(req): Json<RequestBody>,
) -> Result<Json<ResponseBody>, StatusCode> {
    let res = state.device_channel.query(req).await.map_err(|e| {
        log::error!("Failed to query device: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn command(
    State(state): State<AppState>,
    Json(req): Json<RequestBody>,
) -> Result<(), StatusCode> {
    state.device_channel.send(req).map_err(|e| {
        log::error!("Failed to send command: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(())
}
