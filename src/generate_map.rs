extern crate rand;
use self::rand::Rng;

use tile;

use super::minefield::{MAP_SIZE_WIDTH, MAP_SIZE_HEIGHT};

pub fn generate(
  pos_x: usize,
  pos_y: usize,
) -> [[tile::Tile; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT] {
  let mut minefield = [[tile::Tile::new(0); MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT];

  let mut rng = rand::thread_rng();

  // Generate all the mines, placing them randomly around the map.
  for i in 0..MAP_SIZE_WIDTH {
    for j in 0..MAP_SIZE_HEIGHT {
      minefield[i][j].is_mine = rng.gen_range(1, 6) == 1;
    }
  }

  // Make sure the initial tile isn't a mine.
  minefield[pos_x][pos_y].is_mine = false;

  // Find alll the numbers for each tile
  for i in 0..MAP_SIZE_WIDTH {
    for j in 0..MAP_SIZE_HEIGHT {
      if pos_x == i && pos_y == j {
        minefield[i][j].number = 0;
      } else if !minefield[i][j].is_mine {
        minefield[i][j].number = count_mines_around(minefield, i, j);
      }
    }
  }

  minefield
}

/// Given a minefield and a position, determine the amount of mines around the given tile.
fn count_mines_around(
  minefield: [[tile::Tile; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT],
  x: usize,
  y: usize,
) -> i8 {
  // All this crazyness below is to make sure that we don't ever go outside of the map (ie, on
  // corners there'll be issues with tiles that are outside the map in the 3x3 around a tile.)
  let mut total = get_minefield_number(minefield, x+1, y) +
                  get_minefield_number(minefield, x+1, y+1) +
                  get_minefield_number(minefield, x, y+1);

  if x >= 1 {
    total += get_minefield_number(minefield, x-1, y+1) +
             get_minefield_number(minefield, x-1, y);
  }
  if y >= 1 {
    total += get_minefield_number(minefield, x, y-1) +
             get_minefield_number(minefield, x+1, y-1);
  }

  if x >= 1 && y >= 1 {
    total += get_minefield_number(minefield, x-1, y-1);
  }

  total
}

/// Given a minefield position, return the number at a given tile of the field.
fn get_minefield_number(
  minefield: [[tile::Tile; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT],
  x: usize,
  y: usize,
) -> i8 {
  if x < MAP_SIZE_WIDTH && y < MAP_SIZE_HEIGHT && minefield[x][y].is_mine {
    1
  } else {
    0
  }
}
