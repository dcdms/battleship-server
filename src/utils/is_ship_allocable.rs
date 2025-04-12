use crate::{Board, Cell, Direction};

pub fn execute(
  board: &Board,
  origin_index: i32,
  direction: &Direction,
  length: i32,
) -> Option<Vec<i32>> {
  let (cell_offset, barrier_offset) = match direction {
    Direction::Up => (-10, 1),
    Direction::Right => (1, 10),
    Direction::Down => (10, 1),
    Direction::Left => (-1, 10),
  };

  let last_cell_index = origin_index + cell_offset * (length - 1);

  if matches!(direction, Direction::Right | Direction::Left)
    && origin_index.div_floor(10) != last_cell_index.div_floor(10)
  {
    return None;
  }

  if board
    .get((origin_index - cell_offset) as usize)
    .is_some_and(|cell| matches!(cell, Cell::Ship))
    || board
      .get((origin_index - cell_offset - barrier_offset) as usize)
      .is_some_and(|cell| matches!(cell, Cell::Ship))
    || board
      .get((origin_index - cell_offset + barrier_offset) as usize)
      .is_some_and(|cell| matches!(cell, Cell::Ship))
    || board
      .get((last_cell_index + cell_offset) as usize)
      .is_some_and(|cell| matches!(cell, Cell::Ship))
    || board
      .get((last_cell_index + cell_offset - barrier_offset) as usize)
      .is_some_and(|cell| matches!(cell, Cell::Ship))
    || board
      .get((last_cell_index + cell_offset + barrier_offset) as usize)
      .is_some_and(|cell| matches!(cell, Cell::Ship))
  {
    return None;
  }

  let mut cells = Vec::with_capacity(length as usize);

  while cells.len() < length as usize {
    let target_index = origin_index + cell_offset * (cells.len() as i32);

    if board
      .get(target_index as usize)
      .is_none_or(|cell| matches!(cell, Cell::Ship))
      || board
        .get((target_index + barrier_offset) as usize)
        .is_some_and(|cell| matches!(cell, Cell::Ship))
      || board
        .get((target_index - barrier_offset) as usize)
        .is_some_and(|cell| matches!(cell, Cell::Ship))
    {
      return None;
    }

    cells.push(target_index)
  }

  Some(cells)
}
