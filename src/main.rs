extern crate ncurses;
use ncurses::*;

mod tile;
mod generate_map;
mod minefield;

use minefield::{MAP_SIZE_WIDTH, MAP_SIZE_HEIGHT};

fn main() {
  // Generate the initial minefield
  let mut minefield = [[tile::Tile::new(0); MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT];
  let mut minefield_generated = false;

  /* Start ncurses. */
  initscr();

  // Get the cursor position
  let mut pos_x: i32 = 0;
  let mut pos_y: i32 = 5;

  // While this is true, keep looping through the mainloop.
  let mut is_running = true;

  while is_running {
    // Get the screen bounds.
    let max_x: i32 = MAP_SIZE_WIDTH as i32;
    let max_y: i32 = MAP_SIZE_HEIGHT as i32;
    /* getmaxyx(stdscr(), &mut max_y, &mut max_x); */

    // initialize colors
    start_color();
    tile::initialize_tile_colors();

    // line by line, render the minefield
    for y in 0..max_y {
      mv(y, 0);

      // The final line!
      if y == max_y - 1 {
        printw(":");
      } else {
        // All other lines.
        /* printw(&n_chars(max_x, ' ')); */
        for i in 0..max_x {
          mv(y, i);
          let mine = minefield[i as usize][y as usize];
          let color = mine.get_color();

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
    }

    // Draw the cursor
    mv(pos_y, pos_x);


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
          minefield = generate_map::generate(pos_x as usize, pos_y as usize);
          minefield_generated = true;
        }

        // Mark the mine as discovered
        if minefield[pos_x as usize][pos_y as usize].is_flagged == false {
          minefield[pos_x as usize][pos_y as usize].is_discovered = true;
        }

        // If there was a bomb on that tile, stop the game.
        if minefield[pos_x as usize][pos_y as usize].is_mine {
          is_running = false;
        }

        /* // If the tile was a zero, propegate the zero through and clear the rest of the zeros */
        /* // around it. */
        /* if minefield[pos_x as usize][pos_y as usize].number == 0 { */
        /*   propegate_zeros_through_minefield(&mut minefield, pos_x as usize, pos_y as usize); */
        /* } */
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

  // Cleanup ncurses
  endwin();
}

/* fn propegate_zeros_through_minefield( */
/*   &mut minefield: [[tile::Tile; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT], */
/*   initial_x: usize, */
/*   initial_y: usize, */
/* ) -> [[tile::Tile; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT] { */
/*   minefield[initial_x][initial_y].is_discovered = true; */
/*  */
/*   if minefield[initial_x+1][initial_y-1] == 0 { */
/*     propegate_zeros_through_minefield(minefield, initial_x+1, initial_y+1) */
/*   } */
/* } */
