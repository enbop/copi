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
pub async fn fetch(
    State(state): State<AppState>,
    Json(req): Json<RequestBody>,
) -> Result<Json<ResponseBody>, StatusCode> {
    let res = state.device_channel.fetch(req).await.map_err(|e| {
        log::error!("Failed to fetch: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn send(
    State(state): State<AppState>,
    Json(req): Json<RequestBody>,
) -> Result<(), StatusCode> {
    state.device_channel.send(req).map_err(|e| {
        log::error!("Failed to fetch: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(())
}
