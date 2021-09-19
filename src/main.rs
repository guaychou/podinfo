use podinfo::error::handle_error;
use podinfo::startup::startup;
use podinfo::telemetry::{get_subscriber, init_subscriber};
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let subscriber = get_subscriber("podinfo".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let apps_route = startup()
        .layer(
            ServiceBuilder::new()
                .buffer(50)
                .concurrency_limit(25)
                .rate_limit(25, Duration::from_secs(5))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .handle_error(handle_error)
        .check_infallible();

    axum::Server::bind(&addr)
        .serve(apps_route.into_make_service())
        .await?;
    Ok(())
}