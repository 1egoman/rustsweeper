use tile;

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

      // If the square isn't a bomb, then "discover" it.
      if !minefield[x as usize][y as usize].is_mine {
        minefield[x as usize][y as usize].is_discovered = true;
      }

      // If the surrounding square is also a zero, then run this function on it, too.
      if minefield[x as usize][y as usize].number == 0 {
        propegate_zeros_through_minefield(&mut minefield, initial_x+1, initial_y+1);
      }
    }
  }
}
