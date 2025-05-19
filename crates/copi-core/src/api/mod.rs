use crate::AppState;
use crate::generated::RequestBody;
use axum::body::Body;
use axum::extract::FromRequest;
use axum::http::Request;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use http_body_util::BodyExt as _;

// TODO: Uncomment and implement these modules as needed
// pub mod gpio;
// pub mod pio;
pub mod playground;
// pub mod pwm;

pub enum BodyFormat<T> {
    Protobuf(T),
    Json(T),
}

impl<S, T> FromRequest<S> for BodyFormat<T>
where
    S: Send + Sync,
    T: prost::Message + Default + Send,
    Json<T>: FromRequest<S>,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if content_type.contains("application/protobuf")
            || content_type.contains("application/x-protobuf")
        {
            let body = req.into_body();
            let bytes = body
                .collect()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .to_bytes();

            let protobuf_body = T::decode(&bytes[..]).map_err(|_| StatusCode::BAD_REQUEST)?;

            Ok(BodyFormat::Protobuf(protobuf_body))
        } else {
            let Json(body) = Json::<T>::from_request(req, state)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            Ok(BodyFormat::Json(body))
        }
    }
}

pub struct ProtoBufResponse<T>(pub T);

impl<T> IntoResponse for ProtoBufResponse<T>
where
    T: prost::Message,
{
    fn into_response(self) -> Response {
        match self.0.encode_to_vec() {
            data => {
                let headers = [(CONTENT_TYPE, "application/protobuf")];
                (headers, data).into_response()
            }
        }
    }
}

#[axum::debug_handler]
pub async fn query(
    State(state): State<AppState>,
    body_format: BodyFormat<RequestBody>,
) -> Result<impl IntoResponse, StatusCode> {
    let is_protobuf = matches!(body_format, BodyFormat::Protobuf(_));
    let req = match body_format {
        BodyFormat::Json(req) => req,
        BodyFormat::Protobuf(req) => req,
    };

    let res = state.device_channel.query(req).await.map_err(|e| {
        log::error!("Failed to query device: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let resp = if is_protobuf {
        ProtoBufResponse(res).into_response()
    } else {
        Json(res).into_response()
    };
    Ok(resp)
}

#[axum::debug_handler]
pub async fn command(
    State(state): State<AppState>,
    body_format: BodyFormat<RequestBody>,
) -> Result<(), StatusCode> {
    let req = match body_format {
        BodyFormat::Json(req) => req,
        BodyFormat::Protobuf(req) => req,
    };

    state.device_channel.send(req).map_err(|e| {
        log::error!("Failed to send command: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(())
}
