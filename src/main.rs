extern crate ncurses;
use ncurses::*;

mod tile;
mod generate_map;
mod minefield;

use minefield::{
  MAP_SIZE_WIDTH,
  MAP_SIZE_HEIGHT,
  propegate_zeros_through_minefield,
  render,
};

fn main() {
  // Generate the initial minefield
  let mut minefield = [[tile::Tile::new(0); MAP_SIZE_HEIGHT]; MAP_SIZE_WIDTH];
  let mut minefield_generated = false;

  /* Start ncurses. */
  initscr();

  // initialize colors
  start_color();
  tile::initialize_tile_colors();

  // Get the cursor position
  let mut pos_x = MAP_SIZE_WIDTH / 2;
  let mut pos_y = MAP_SIZE_HEIGHT / 2;

  // While this is true, keep looping through the mainloop.
  let mut is_running = true;

  while is_running {
    // Render the minefield.
    render(minefield);

    // Draw the cursor
    mv(pos_y as i32, pos_x as i32);


    // log stuff out!
    /* mv(0, 0); */
    /* printw(&format!("Position Y: {}", pos_y)); */

    let character = getch();
    mv(0, 0);
    printw(&format!("Character: {}", character));
    match character {
      // Selecting a tile (the enter key). Can only select non flagged tiles.
      10 => {
        // If this is the first mine to be hit, generate the minefield
        if !minefield_generated {
          minefield = generate_map::generate(pos_x, pos_y);
          minefield_generated = true;
        }

        // Mark the mine as discovered
        if minefield[pos_x][pos_y].is_flagged == false {
          minefield[pos_x][pos_y].is_discovered = true;
        }

        // If there was a bomb on that tile, stop the game.
        if minefield[pos_x][pos_y].is_mine {
          is_running = false;
        }

        // If the tile was a zero, propegate the zero through and clear the rest of the zeros
        // around it.
        if minefield[pos_x as usize][pos_y as usize].number == 0 {
          propegate_zeros_through_minefield(&mut minefield, pos_x as usize, pos_y as usize);
        }
      },

      // Movement
      // TODO: Make sure we stay within the bounds
      106 => pos_y += 1,
      107 => pos_y -= 1,
      108 => pos_x += 1,
      104 => pos_x -= 1,

      // The f key to flag something as a bomb
      // It only works on cells that haven't been previously discovered.
      // If a cell isn't discovered, flag it.
      102 => {
        let mine = minefield[pos_x as usize][pos_y as usize];
        if minefield_generated && minefield[pos_x as usize][pos_y as usize].is_discovered == false {
          minefield[pos_x as usize][pos_y as usize].is_flagged = !mine.is_flagged;
        }
      }
      _ => (),
    };

    refresh();
  }

  // At this point, the user stepped on a mine :/
  // So...

  // Make the whole minefield visible
  for x in 0..MAP_SIZE_WIDTH {
    for y in 0..MAP_SIZE_HEIGHT {
      minefield[x][y].is_discovered = true;
    }
  }

  // Render the minefield, showing all the parts of the field.
  render(minefield);

  // Wait for the user to press a key
  getch();

  // Cleanup ncurses
  endwin();

  // And we're done!
}
