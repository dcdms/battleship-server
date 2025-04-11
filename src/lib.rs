use serde::Serialize;

pub mod handlers;
pub mod utils;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl rand::distr::Distribution<Direction> for rand::distr::StandardUniform {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
    match rng.random_range(0..=3) {
      0 => Direction::Up,
      1 => Direction::Right,
      2 => Direction::Down,
      _ => Direction::Left,
    }
  }
}

#[derive(Serialize, Clone, Debug)]
pub struct Ship {
  pub starts_at: u8,
  pub direction: Direction,
  pub length: u8,
}

#[derive(Serialize, Clone, Debug)]
pub struct Player {
  pub id: u32,
  pub filled_board_cells: Vec<u8>,
  pub ships: Vec<Ship>,

  #[serde(skip_serializing)]
  pub tx: tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Room {
  pub id: u32,
  pub players: Vec<Player>,

  #[serde(skip_serializing)]
  pub next_player_id: u32,
}

#[derive(Serialize, Clone, Debug)]
pub struct State {
  pub rooms: Vec<Room>,
  pub next_room_id: u32,
}

impl Default for State {
  fn default() -> Self {
    Self {
      rooms: vec![],
      next_room_id: 1,
    }
  }
}

#[derive(Serialize)]
pub struct RoomEnteredEvent {
  pub ships: Vec<Ship>,
  pub has_opponent: bool,
}

#[derive(Serialize)]
pub struct OpponentEnteredEvent {}

#[derive(Serialize)]
#[serde(tag = "event", content = "data")]
pub enum WebSocketSentEvent {
  #[serde(rename = "room.entered")]
  RoomEntered(RoomEnteredEvent),

  #[serde(rename = "opponent.entered")]
  OpponentEntered(OpponentEnteredEvent),
}
