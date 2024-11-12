use tracing::info;
use ygo_draft_server::{
    create_router,
    routes::shutdown
};

#[tokio::main]
async fn main() {
    let router = create_router().await;
    info!("router created");

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await.expect("TcpListener should work w/ hard-coded localhost");
    info!("listening on TCPListenter: {:?}", tcp_listener);

    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("Must be able to run the server (std::io::error)");
}