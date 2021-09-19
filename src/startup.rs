use crate::handler::*;
use axum::{handler::get, routing::BoxRoute, Router};

pub fn startup() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(root))
        .route("/notfound", get(fourofour))
        .route("/internalservererror", get(internalservererror))
        .route("/health", get(health))
        .route("/terminate", get(terminate))
        .boxed()
}
