use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::convert::Infallible;
use tower::BoxError;

pub fn handle_error(error: BoxError) -> Result<impl IntoResponse, Infallible> {
    if error.is::<tower::timeout::error::Elapsed>() {
        return Ok((
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "code" : 408,
                "message" : "Uhh ohh, request time out",
            })),
        ));
    };

    if error.is::<tower::load_shed::error::Overloaded>() {
        return Ok((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "code" : 503,
                "message" : "Uhh ohh, service unavailable",
            })),
        ));
    }

    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "code" : 500,
            "message" : "Uhh ohh, unhandled internal error",
        })),
    ))
}
