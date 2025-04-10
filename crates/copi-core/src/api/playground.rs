use axum::{
    http::{StatusCode, header},
    response::IntoResponse,
};

#[derive(rust_embed::Embed)]
#[folder = "./static"]
struct StaticFiles;

pub async fn playground() -> impl IntoResponse {
    serve_static_file("playground/index.html").await
}

async fn serve_static_file(path: &str) -> impl IntoResponse {
    match StaticFiles::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}
