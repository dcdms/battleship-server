use crate::{
  utils::generate_random_board, Cell, OpponentCellHittedEvent, OpponentEnteredEvent, OpponentLeftEvent, Player, RoomEnteredEvent, State, WebSocketReceivedEvent, WebSocketSentEvent
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
    .any(|r| r.id == room_id_as_u32 && r.players.len() < 2)
  {
    return (StatusCode::NOT_FOUND, "room_not_found").into_response();
  }

  ws.on_upgrade(move |socket| websocket(socket, state, room_id_as_u32))
}

async fn websocket(stream: WebSocket, state: Arc<RwLock<State>>, room_id: u32) {
  let (mut sender, mut receiver) = stream.split();

  let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

  let player_id = state
    .read()
    .await
    .rooms
    .iter()
    .find(|r| r.id == room_id)
    .unwrap()
    .next_player_id;

  let player = Player {
    id: player_id,
    board: generate_random_board::execute(),
    tx: tx.clone(),
  };

  let room_player = player.clone();

  {
    let mut writer = state.write().await;

    let room = writer.rooms.iter_mut().find(|r| r.id == room_id).unwrap();

    room.players.push(room_player);
    room.next_player_id += 1;
    
    if room.turn.is_none() {
      room.turn = Some(player_id)
    }
  };

  {
    let reader = state.read().await;

    let room = reader
    .rooms
    .iter()
    .find(|r| r.id == room_id)
    .unwrap();

    let opponent = room
      .players
      .iter()
      .find(|p| p.id != player_id);

    if let Some(opponent) = opponent {
      let _ = opponent.tx.send(Message::text(
        serde_json::to_string(&WebSocketSentEvent::OpponentEntered(
          OpponentEnteredEvent {},
        ))
        .unwrap(),
      ));
    }

    let _ = tx.send(Message::text(
      serde_json::to_string::<WebSocketSentEvent>(
        &WebSocketSentEvent::RoomEntered(RoomEnteredEvent {
          board: player.board,
          has_opponent: opponent.is_some(),
          has_turn: room.turn.unwrap() == player_id,
        }),
      )
      .unwrap(),
    ));
  }

  let mut send_task = tokio::spawn(async move {
    while let Some(message) = rx.recv().await {
      if sender.send(message).await.is_err() {
        break;
      }
    }
  });

  let tx_clone = tx.clone();
  let state_clone = state.clone();

  let mut recv_task = tokio::spawn(async move {
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
      let event =
        match serde_json::from_str::<WebSocketReceivedEvent>(text.as_str()) {
          Ok(event) => event,
          Err(_) => continue,
        };

      match event {
        WebSocketReceivedEvent::CellChosen(event) => {
          if event.index > 99 {
            return;
          }

          let mut writer = state_clone.write().await;

          let opponent = writer
            .rooms
            .iter_mut()
            .find(|r| r.id == room_id)
            .unwrap()
            .players
            .iter()
            .find(|p| p.id != player_id)
            .unwrap();

          let _ = opponent.tx.send(Message::Text(text));

          let mut cell = player.board[event.index as usize];

          match cell {
            Cell::Empty => cell = Cell::Hitted,
            Cell::Ship => cell = Cell::HittedShip,
            _ => {},
          };

          let _ = tx_clone.send(Message::text(
            serde_json::to_string(&WebSocketSentEvent::OpponentCellHitted(
              OpponentCellHittedEvent {
                index: event.index,
                hitted_ship: matches!(cell, Cell::Hitted | Cell::HittedShip),
              },
            ))
            .unwrap(),
          ));
        }
      }
    }
  });

  tokio::select! {
    _ = &mut send_task => recv_task.abort(),
    _ = &mut recv_task => send_task.abort(),
  }

  {
    let mut writer = state.write().await;

    let room = writer
      .rooms
      .iter_mut()
      .find(|r| r.id == room_id)
      .unwrap();

    if let Some(opponent) = room.players.iter().find(|p| p.id != player_id) {
      let _ = opponent.tx.send(Message::text(
        serde_json::to_string(&WebSocketSentEvent::OpponentLeft(
          OpponentLeftEvent {},
        ))
        .unwrap(),
      ));

      if room.turn.is_some_and(|turn| turn == player_id) {
        room.turn = Some(opponent.id)
      }
    }

    room.players.retain(|p| p.id != player_id);
  };
}
