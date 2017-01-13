use tile;

extern crate ncurses;
use ncurses::*;

pub type Minefield = [[tile::Tile; MAP_SIZE_HEIGHT]; MAP_SIZE_WIDTH];

// Assemble a minefield
pub const MAP_SIZE_WIDTH: usize = 60;
pub const MAP_SIZE_HEIGHT: usize = 30;

/// Is the given x and y coordinate a valid square in the minefield?
fn is_in_minefield(x: i32, y: i32) -> bool {
  if x >= 0 && y >= 0 {
    (x as usize) < MAP_SIZE_WIDTH && (y as usize) < MAP_SIZE_HEIGHT
  } else {
    false
  }
}

pub fn propegate_zeros_through_minefield(
  mut minefield: &mut Minefield,
  initial_x: usize,
  initial_y: usize,
) {
  // If the current tile isn't a mine, then uncover it.
  if !minefield[initial_x][initial_y].is_mine {
    minefield[initial_x][initial_y].is_discovered = true;
  }

  // Coerse to i32 so these guys can (potentially) go below zero later.
  let x: i32 = initial_x as i32;
  let y: i32 = initial_y as i32;

  // The "plus" of squares around the current square.
  let positions = vec![
    (x+1, y+0),
    (x-1, y+0),
    (x+0, y+1),
    (x+0, y-1),
  ];

  // For each square in the plus around the passed square...
  for &(x, y) in positions.iter() {
    if is_in_minefield(x, y) {

      // discover all squares outside the zero filled region.
      if minefield[x as usize][y as usize].number > 0 {
        minefield[x as usize][y as usize].is_discovered = true;
      }


      // If the surrounding square is also a zero, then run this function on it, too.
      if minefield[x as usize][y as usize].number == 0 &&
        !minefield[x as usize][y as usize].is_discovered {
        propegate_zeros_through_minefield(&mut minefield, x as usize, y as usize);
      }
    }
  }
}

pub fn render(minefield: Minefield) {
  // line by line, render the minefield
  for x in 0..MAP_SIZE_WIDTH {
    for y in 0..MAP_SIZE_HEIGHT {
      let mine = minefield[x as usize][y as usize];
      let color = mine.get_color();

      // Move the the position where that mine will be drawn
      mv(y as i32, x as i32);

      let mut total = String::new();
      total.push(mine.get_repr());

      // Render the item in color, if there was no color, then just render it.
      match color {
        Some(color) => {
          attron(color);
          printw(&total);
          attroff(color);
        }
        None => {
          printw(&total);
        }
      }
    }
  }

  // The final line!
  mv(MAP_SIZE_HEIGHT as i32, 0);
  printw(":");
}
