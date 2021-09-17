use podinfo::startup::startup;
use podinfo::telemetry::{get_subscriber, init_subscriber};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_log::log::error;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("podinfo".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let apps_route = startup().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    );

    let server = axum::Server::bind(&addr)
        .serve(apps_route.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok();
        });

    // Await the `server` receiving the signal...
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    // And later, trigger the signal by calling `tx.send(())`.
    let _ = tx.send(());
}