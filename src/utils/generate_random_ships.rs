use std::collections::HashSet;

use crate::{Direction, Ship};
use rand::{
  Rng,
  seq::{IndexedRandom, IteratorRandom},
};

pub fn execute() -> Vec<Ship> {
  let mut available_starts_at: Vec<u8> = (0..100).collect();
  let mut available_lengths = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 4];
  let mut result: Vec<Ship> = vec![];

  let mut rng = rand::rng();

  while !available_lengths.is_empty() {
    let direction: Direction = rng.random();

    let length_index = rng.random_range(0..available_lengths.len());
    let length = available_lengths.swap_remove(length_index);

    let starts_at = match length {
      1 => available_starts_at.choose(&mut rng).unwrap(),
      l => match direction {
        Direction::Up => available_starts_at
          .iter()
          .filter(|&&x| x >= (10 * (l - 1)))
          .choose(&mut rng)
          .unwrap(),
        Direction::Right => available_starts_at
          .iter()
          .filter(|&x| x % 10 <= 10 - l)
          .choose(&mut rng)
          .unwrap(),
        Direction::Down => available_starts_at
          .iter()
          .filter(|&&x| x < (99 - (10 * (l - 1))))
          .choose(&mut rng)
          .unwrap(),
        Direction::Left => available_starts_at
          .iter()
          .filter(|&x| x % 10 >= l - 1)
          .choose(&mut rng)
          .unwrap(),
      },
    };

    let mut filled = HashSet::new();
    let mut offset = 0;

    while offset < length {
      let index = match direction {
        Direction::Up => starts_at - 10 * offset,
        Direction::Right => starts_at + offset,
        Direction::Down => starts_at + 10 * offset,
        Direction::Left => starts_at - offset,
      };

      if index % 10 == 0 {
        filled.extend([index, index + 1, index + 10, index + 11]);

        if index > 0 {
          filled.extend([index - 10, index - 9]);
        }
      } else if index == 9 {
        filled.extend([index - 1, index, index + 9, index + 10]);

        if index > 9 {
          filled.extend([index - 11, index - 10]);
        }
      } else {
        filled.extend([
          index - 1,
          index,
          index + 1,
          index + 9,
          index + 10,
          index + 11,
        ]);

        if index > 10 {
          filled.extend([index - 11, index - 10, index - 9])
        }
      }

      offset += 1;
    }

    result.push(Ship {
      starts_at: *starts_at,
      direction,
      length,
    });

    available_starts_at.retain(|s| filled.iter().any(|f| f != s))
  }

  result
}
