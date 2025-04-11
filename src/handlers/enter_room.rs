use crate::{
  JoinedRoomEvent, Player, State, WebSocketSentEvent,
  utils::generate_random_ships,
};
use axum::{
  extract::{
    Path, State as StateExtractor,
    ws::{Message, WebSocket, WebSocketUpgrade},
  },
  http::StatusCode,
  response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};

pub async fn handle(
  ws: WebSocketUpgrade,
  Path(room_id): Path<String>,
  StateExtractor(state): StateExtractor<Arc<RwLock<State>>>,
) -> impl IntoResponse {
  let room_id_as_u32: u32 = match room_id.parse() {
    Ok(room_id) => room_id,
    Err(_) => {
      return (StatusCode::UNPROCESSABLE_ENTITY, "invalid_room_id")
        .into_response();
    }
  };

  if !state
    .read()
    .await
    .rooms
    .iter()
    .any(|r| r.id == room_id_as_u32)
  {
    return (StatusCode::NOT_FOUND, "room_not_found").into_response();
  }

  ws.on_upgrade(move |socket| websocket(socket, state, room_id_as_u32))
}

async fn websocket(stream: WebSocket, state: Arc<RwLock<State>>, room_id: u32) {
  let (mut sender, mut receiver) = stream.split();

  let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

  let mut state_write_guard = state.write().await;

  let room = state_write_guard
    .rooms
    .iter_mut()
    .find(|r| r.id == room_id)
    .unwrap();

  let player = Player {
    id: room.next_player_id,
    filled_board_cells: vec![],
    ships: generate_random_ships::execute(),
    tx: tx.clone(),
  };

  room.players.push(player.clone());
  room.next_player_id += 1;

  let _ = tx.send(Message::text(
    serde_json::to_string::<WebSocketSentEvent>(&WebSocketSentEvent::Joined(
      JoinedRoomEvent { player },
    ))
    .unwrap(),
  ));

  let mut send_task = tokio::spawn(async move {
    while let Some(message) = rx.recv().await {
      if sender.send(message).await.is_err() {
        break;
      }
    }
  });

  let tx_clone = tx.clone();

  let mut recv_task = tokio::spawn(async move {
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
      let _ = tx_clone
        .send(Message::text(format!("message_from_recv_task: {}", text)));
    }
  });

  tokio::select! {
    _ = &mut send_task => recv_task.abort(),
    _ = &mut recv_task => send_task.abort(),
  }

  let _ = tx.send(Message::text("finished"));
}
