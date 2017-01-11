extern crate ncurses;
use ncurses::*;

extern crate rand;
use rand::Rng;

const COLOR_BACKGROUND: i16 = 16;
const PRESET_FLAG: i16 = 1;

// Assemble a minefield
const MAP_SIZE_WIDTH: usize = 10;
const MAP_SIZE_HEIGHT: usize = 10;

/// Create a string that is n characters long all of the given character.
/// n_chars(5, ' ') returns "     " (5 spaces)
fn n_chars(n: i32, character: char) -> String {
  let mut padding = String::new();
  for _ in 0..n {
    padding.push(character);
  }

  padding.clone()
}

/// Given a square of the minefield, return the character to draw for that square.
fn get_mine_character(mine: Square) -> char {
  if mine.is_flagged {
    'F'
  } else if mine.is_discovered && mine.is_mine {
    // If we know it's a bomb, then display that.
    'B'
  } else if mine.is_discovered {
    // Display the number associated with the square
    mine.number.to_string().chars().nth(0).unwrap()
  } else {
    '*'
  }
}

fn get_mine_color(mine: Square) -> Option<i16> {
  if mine.is_flagged {
    Some(PRESET_FLAG)
  } else {
    None
  }
}

fn start_render_loop(minefield: &mut [[Square; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT]) {
  /* Start ncurses. */
  initscr();

  // Get the cursor position
  let mut pos_x = 0;
  let mut pos_y = 5;

  // While this is true, keep looping through the mainloop.
  let mut is_running = true;

  while is_running {
    // Get the screen bounds.
    let max_x: i32 = MAP_SIZE_WIDTH as i32;
    let max_y: i32 = MAP_SIZE_HEIGHT as i32;
    /* getmaxyx(stdscr(), &mut max_y, &mut max_x); */

    start_color();
    init_color(COLOR_BACKGROUND, 0, 0, 0);
    /* init_pair(PRESET_FLAG, COLOR_RED, COLOR_BLACK); */
    init_pair(PRESET_FLAG, COLOR_BLACK, COLOR_RED);
    /* attron(COLOR_PAIR(PRESET_FLAG)); */

    // line by line, render the minefield
    for y in 0..max_y {
      mv(y, 0);

      // The final line!
      if y == max_y - 1 {
        printw(":");
      } else {
        // All other lines.
        /* printw(&n_chars(max_x, ' ')); */
        let mut total = String::new();
        for i in 0..MAP_SIZE_WIDTH {
          let mine = minefield[i as usize][y as usize];
          total.push(get_mine_character(mine));
        }
        printw(&total);
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
        if minefield[pos_x as usize][pos_y as usize].is_flagged == false {
          // Mark the mine as discovered
          minefield[pos_x as usize][pos_y as usize].is_discovered = true;
        }

        // If there was a bomb on that square, stop the game.
        if minefield[pos_x as usize][pos_y as usize].is_mine {
          is_running = false;
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
        if minefield[pos_x as usize][pos_y as usize].is_discovered == false {
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

#[derive(Copy, Clone, Debug)]
struct Square {
  is_discovered: bool,
  number: i8,
  is_mine: bool,
  is_flagged: bool,
}

/// Given a minefield position, return the number at a given square of the field.
fn get_minefield_number(
  minefield: [[Square; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT],
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
  minefield: [[Square; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT],
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

fn main() {
  let mut minefield = [[Square {
    is_discovered: false,
    number: 0,
    is_mine: false,
    is_flagged: false,
  }; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT];

  let mut rng = rand::thread_rng();

  // Generate all the mines, placing them randomly around the map.
  for i in 0..MAP_SIZE_WIDTH {
    for j in 0..MAP_SIZE_HEIGHT {
      minefield[i][j].is_mine = rng.gen();
    }
  }

  // Find alll the numbers for each square
  for i in 0..MAP_SIZE_WIDTH {
    for j in 0..MAP_SIZE_HEIGHT {
      minefield[i][j].number = count_mines_around(minefield, i, j);
    }
  }

  start_render_loop(&mut minefield);
}
