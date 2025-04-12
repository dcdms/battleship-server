use crate::{Room, State};
use axum::extract::{Json, State as StateExtractor};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize)]
pub struct CreateRoomResponse {
  room_id: u32,
}

pub async fn handle(
  StateExtractor(state): StateExtractor<Arc<RwLock<State>>>,
) -> Json<CreateRoomResponse> {
  let room_id = state.read().await.next_room_id;

  {
    let mut writer = state.write().await;

    writer.rooms.push(Room {
      id: room_id,
      next_player_id: 1,
      players: vec![],
      turn: None,
    });

    writer.next_room_id += 1;
  };

  Json(CreateRoomResponse { room_id })
}
