use crate::{Board, Cell, Direction};
use rand::Rng;

use super::is_ship_allocable;

pub fn execute() -> Board {
  let mut board = [Cell::Empty; 100];
  let mut available_lengths = vec![4, 3, 3, 2, 2, 2, 1, 1, 1, 1];

  let mut rng = rand::rng();

  while !available_lengths.is_empty() {
    let direction: Direction = rng.random();
    let length = available_lengths.remove(0);

    let mut origin_index = rng.random_range(0..100);

    let fitted = loop {
      if let Some(fitted) =
        is_ship_allocable::execute(&board, origin_index, &direction, length)
      {
        break fitted;
      } else {
        origin_index = rng.random_range(0..100);
      }
    };

    for cell_index in fitted {
      board[cell_index as usize] = Cell::Ship;
    }
  }

  board
}
