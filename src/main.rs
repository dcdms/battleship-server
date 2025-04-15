use axum::{
  Router,
  routing::{get, post},
};
use server::{
  State,
  handlers::{create_room, enter_room},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  let state = Arc::new(RwLock::new(State::default()));

  let router = Router::new()
    .route("/rooms/{room_id}", get(enter_room::handle))
    .route("/rooms", post(create_room::handle))
    .layer(
      CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any),
    )
    .with_state(state);

  Ok(router.into())
}
