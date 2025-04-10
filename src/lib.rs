pub struct Ship {
  pub starts_at: u8,
  pub is_horizontal: bool,
  pub length: u8,
}

pub struct Player {
  pub id: u8,
  pub board_cells: [bool; 100],
  pub ships: [Ship; 10],
}

pub struct Room {
  pub id: u32,
  pub players: Vec<Player>,
}

pub struct AppState {
  pub rooms: Vec<Room>,
  pub room_id_counter: u32,
  pub tx: tokio::sync::broadcast::Sender<String>,
}
