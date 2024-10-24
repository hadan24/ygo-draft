use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use crate::card::{CardPool, DraftOptions};


pub async fn handler_root() -> Response {
    (StatusCode::OK, "Hallo :D 🦀").into_response()
}

pub async fn get_main_opts(State(pool): State<CardPool>)
    -> Json<DraftOptions>
{
    Json(CardPool::generate_draft_options(&pool.main_deck))
}

pub async fn get_extra_opts(State(pool): State<CardPool>)
    -> Json<DraftOptions>
{
    Json(CardPool::generate_draft_options(&pool.extra_deck))
}

pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}