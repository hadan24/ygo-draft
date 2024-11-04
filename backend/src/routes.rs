use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use tracing::{info, info_span};
use crate::card::{CardPool, DraftOptions};


pub async fn handler_root() -> Response {
    (StatusCode::OK, "Hallo :D 🦀").into_response()
}

pub async fn get_main_opts(State(pool): State<CardPool>)
    -> Json<DraftOptions>
{
    let main_opts_span = info_span!("main_opts");
    let _enter = main_opts_span.enter();

    info!("getting main deck options");
    Json(CardPool::generate_draft_options(&pool.main_deck))
}

pub async fn get_extra_opts(State(pool): State<CardPool>)
    -> Json<DraftOptions>
{
    let extra_opts_span = info_span!("main_opts");
    let _enter = extra_opts_span.enter();

    info!("getting extra deck options");
    Json(CardPool::generate_draft_options(&pool.extra_deck))
}

pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}