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

#[tokio::main]
async fn main() {
  let state = Arc::new(RwLock::new(State::default()));

  let app = Router::new()
    .route("/rooms/{room_id}", get(enter_room::handle))
    .route("/rooms", post(create_room::handle))
    .with_state(state.clone());

  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

  axum::serve(listener, app).await.unwrap();
}
