extern crate ncurses;
use ncurses::*;

extern crate rand;
use rand::Rng;

mod minemap;

// Assemble a minefield
const MAP_SIZE_WIDTH: usize = 20;
const MAP_SIZE_HEIGHT: usize = 20;


#[derive(Copy, Clone, Debug)]
struct Square {
  is_discovered: bool,
  number: i8,
  is_mine: bool,
  is_flagged: bool,
}

/// Given a minefield position, return the number at a given square of the field.
fn get_minefield_number(
  minefield: [[minemap::Square; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT],
  x: usize,
  y: usize,
) -> i8 {
  if x < MAP_SIZE_WIDTH && y < MAP_SIZE_HEIGHT && minefield[x][y].is_mine {
    1
  } else {
    0
  }
}

/// Given a minefield and a position, determine the amount of mines around the given tile.
fn count_mines_around(
  minefield: [[minemap::Square; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT],
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

fn generate_minefield(pos_x: usize, pos_y: usize) -> [[minemap::Square; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT] {
  let mut minefield = [[minemap::Square {
    is_discovered: false,
    number: 0,
    is_mine: false,
    is_flagged: false,
  }; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT];

  let mut rng = rand::thread_rng();

  // Generate all the mines, placing them randomly around the map.
  for i in 0..MAP_SIZE_WIDTH {
    for j in 0..MAP_SIZE_HEIGHT {
      minefield[i][j].is_mine = rng.gen_range(1, 6) == 1;
    }
  }

  // Make sure the initial square isn't a mine.
  minefield[pos_x][pos_y].is_mine = false;

  // Find alll the numbers for each square
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

fn main() {
  // Generate the initial minefield
  let mut minefield = [[minemap::Square {
    is_discovered: false,
    number: 0,
    is_mine: false,
    is_flagged: false,
  }; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT];
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
    minemap::initialize_square_colors();

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
      // Selecting a square (the enter key). Can only select non flagged squares.
      10 => {
        // If this is the first mine to be hit, generate the minefield
        if !minefield_generated {
          minefield = generate_minefield(pos_x as usize, pos_y as usize);
          minefield_generated = true;
        }

        // Mark the mine as discovered
        if minefield[pos_x as usize][pos_y as usize].is_flagged == false {
          minefield[pos_x as usize][pos_y as usize].is_discovered = true;
        }

        // If there was a bomb on that square, stop the game.
        if minefield[pos_x as usize][pos_y as usize].is_mine {
          is_running = false;
        }

        /* // If the square was a zero, propegate the zero through and clear the rest of the zeros */
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
/*   &mut minefield: [[minemap::Square; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT], */
/*   initial_x: usize, */
/*   initial_y: usize, */
/* ) -> [[minemap::Square; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT] { */
/*   minefield[initial_x][initial_y].is_discovered = true; */
/*  */
/*   if minefield[initial_x+1][initial_y-1] == 0 { */
/*     propegate_zeros_through_minefield(minefield, initial_x+1, initial_y+1) */
/*   } */
/* } */
