use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json
};
use tracing::{debug, info, info_span, trace};
use crate::card::{CardPool, DraftOptions};


pub async fn handler_root() -> Response {
    (StatusCode::OK, "Hallo :D 🦀").into_response()
}

pub async fn main_opts(
    State(pool): State<CardPool>,
    headers: HeaderMap,
    body: Bytes
)
    -> Json<DraftOptions>
{
    let main_opts_span = info_span!("get_main_opts");
    let _enter = main_opts_span.enter();
    trace!("headers: {:?}", headers);
    trace!("body: {:?}", body);

    info!("returning requested main deck options");
    Json(CardPool::generate_draft_options(&pool.main_deck))
}

pub async fn extra_opts(
    State(pool): State<CardPool>,
    headers: HeaderMap,
    body: Bytes
)
    -> Json<DraftOptions>
{
    let extra_opts_span = info_span!("get_extra_opts");
    let _enter = extra_opts_span.enter();
    trace!("headers: {:?}", headers);
    trace!("body: {:?}", body);

    info!("returning requested extra deck options");
    Json(CardPool::generate_draft_options(&pool.extra_deck))
}

pub async fn handler_404(headers: HeaderMap, body: Bytes) -> Response {
    debug!("headers: {:?}", headers);
    debug!("body: {:?}", body);

    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}

// taken from axum example:
// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/
pub async fn shutdown() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}