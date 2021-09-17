use axum::{http::StatusCode, response::IntoResponse, Json};
use gethostname::gethostname;
use serde_json::json;
// basic handler that responds with a static string
pub async fn root() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "code" : 200,
            "message" : "Hello world !",
            "hostname": gethostname().into_string().unwrap(),
        })),
    )
}

pub async fn fourofour() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "code" : 404,
            "message" : "Uhh ohh, not found !",
            "hostname": gethostname().into_string().unwrap(),
        })),
    )
}

pub async fn internalservererror() -> impl IntoResponse {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({
            "code" : 503,
            "message" : "Uhh ohh, service is unavailable",
            "hostname": gethostname().into_string().unwrap(),
        })),
    )
}
