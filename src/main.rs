use axum::{
  Router,
  extract::{
    Path, State,
    ws::{Message, WebSocket, WebSocketUpgrade},
  },
  response::IntoResponse,
  routing::get,
};
use futures::{sink::SinkExt, stream::StreamExt};
use server::AppState;
use std::sync::Arc;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
  let (tx, _rx) = broadcast::channel(100);

  let state = Arc::new(AppState {
    rooms: vec![],
    room_id_counter: 0,
    tx,
  });

  let app = Router::new()
    .route("/ws/{room_id}", get(websocket_handler))
    .with_state(state);

  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

  axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
  ws: WebSocketUpgrade,
  Path(room_id): Path<String>,
  State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
  ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
  let (mut sender, mut receiver) = stream.split();

  let mut rx = state.tx.subscribe();

  let _ = state.tx.send(String::from("connected"));

  let mut send_task = tokio::spawn(async move {
    while let Ok(message) = rx.recv().await {
      if sender.send(Message::text(message)).await.is_err() {
        break;
      }
    }
  });

  let tx = state.tx.clone();

  let mut recv_task = tokio::spawn(async move {
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
      let _ = tx.send(format!("message_from_recv_task: {}", text));
    }
  });

  tokio::select! {
    _ = &mut send_task => recv_task.abort(),
    _ = &mut recv_task => send_task.abort(),
  }

  let _ = state.tx.send(String::from("finished"));
}
