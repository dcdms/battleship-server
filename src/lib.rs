#![feature(int_roundings)]

use serde::{Deserialize, Serialize, Serializer, ser::SerializeTuple};

pub mod handlers;
pub mod utils;

#[derive(Debug)]
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

#[derive(Serialize, Clone, Debug, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Cell {
  Empty,
  Hitted,
  Ship,
  HittedShip,
}

type Board = [Cell; 100];

#[derive(Clone, Debug)]
pub struct Player {
  pub id: u32,
  pub board: [Cell; 100],
  pub tx: tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>,
}

#[derive(Clone, Debug)]
pub struct Room {
  pub id: u32,
  pub players: Vec<Player>,
  pub next_player_id: u32,
  pub turn: Option<u32>,
}

#[derive(Clone, Debug)]
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

fn serialize_arr<const N: usize, S, T>(
  t: &[T; N],
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: Serialize,
{
  let mut ser_tuple = serializer.serialize_tuple(N)?;

  for el in t {
    ser_tuple.serialize_element(el)?;
  }

  ser_tuple.end()
}

#[derive(Serialize)]
pub struct RoomEnteredEvent {
  #[serde(serialize_with = "serialize_arr")]
  pub board: [Cell; 100],

  pub has_opponent: bool,
  pub has_turn: bool,
}

#[derive(Serialize)]
pub struct OpponentEnteredEvent {}

#[derive(Serialize)]
pub struct OpponentLeftEvent {}

#[derive(Serialize)]
pub struct OpponentCellHittedEvent {
  pub index: u8,
  pub hitted_ship: bool,
}

#[derive(Serialize)]
#[serde(tag = "event", content = "data")]
pub enum WebSocketSentEvent {
  #[serde(rename = "room.entered")]
  RoomEntered(RoomEnteredEvent),

  #[serde(rename = "opponent.entered")]
  OpponentEntered(OpponentEnteredEvent),

  #[serde(rename = "opponent.left")]
  OpponentLeft(OpponentLeftEvent),

  #[serde(rename = "opponent.cell.hitted")]
  OpponentCellHitted(OpponentCellHittedEvent),
}

#[derive(Deserialize)]
pub struct CellChosenEvent {
  pub index: u8,
}

#[derive(Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum WebSocketReceivedEvent {
  #[serde(rename = "cell.chosen")]
  CellChosen(CellChosenEvent),
}
