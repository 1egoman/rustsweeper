extern crate ncurses;
use ncurses::init_pair;

// Import all colors needed to draw the mine map
use ncurses::COLOR_PAIR;
use ncurses::COLOR_WHITE;
use ncurses::COLOR_RED;
use ncurses::COLOR_GREEN;
use ncurses::COLOR_BLUE;
use ncurses::COLOR_YELLOW;
use ncurses::COLOR_MAGENTA;
use ncurses::COLOR_CYAN;

#[derive(Copy, Clone, Debug)]
pub struct Tile{
  pub is_discovered: bool,
  pub number: i8,
  pub is_mine: bool,
  pub is_flagged: bool,
}

const PRESET_FLAG: i16 = 1;

const PRESET_ONE: i16 = 2;
const PRESET_TWO: i16 = 3;
const PRESET_THREE: i16 = 4;
const PRESET_FOUR: i16 = 5;
const PRESET_FIVE: i16 = 6;
const PRESET_SIX: i16 = 7;

pub fn initialize_tile_colors() {
  init_pair(PRESET_FLAG, COLOR_RED, COLOR_WHITE);

  init_pair(PRESET_ONE, COLOR_BLUE, COLOR_WHITE);
  init_pair(PRESET_TWO, COLOR_GREEN, COLOR_WHITE);
  init_pair(PRESET_THREE, COLOR_YELLOW, COLOR_WHITE);
  init_pair(PRESET_FOUR, COLOR_MAGENTA, COLOR_WHITE);
  init_pair(PRESET_FIVE, COLOR_CYAN, COLOR_WHITE);
  init_pair(PRESET_SIX, COLOR_RED, COLOR_WHITE);
}

impl Tile{
  /// Generate a new number square
  pub fn new(number: i8) -> Tile {
    Tile {
      number: number,
      is_discovered: false,
      is_mine: false,
      is_flagged: false,
    }
  }

  pub fn get_color(self) -> Option<u32> {
    if self.is_flagged {
      Some(COLOR_PAIR(PRESET_FLAG))
    } else if self.is_discovered == false {
      None
    } else if self.number == 1 {
      Some(COLOR_PAIR(PRESET_ONE))
    } else if self.number == 2 {
      Some(COLOR_PAIR(PRESET_TWO))
    } else if self.number == 3 {
      Some(COLOR_PAIR(PRESET_THREE))
    } else if self.number == 4 {
      Some(COLOR_PAIR(PRESET_FOUR))
    } else if self.number == 5 {
      Some(COLOR_PAIR(PRESET_FIVE))
    } else if self.number == 6 {
      Some(COLOR_PAIR(PRESET_SIX))
    } else {
      None
    }
  }

  /// Given a square of the minefield, return the character to draw for that square.
  pub fn get_repr(self) -> char {
    if self.is_flagged {
      'F'
    } else if self.is_discovered && self.is_mine {
      // If we know it's a bomb, then display that.
      'B'
    } else if self.is_discovered && self.number == 0 {
      ' '
    } else if self.is_discovered {
      // Display the number associated with the square
      self.number.to_string().chars().nth(0).unwrap()
    } else {
      '*'
    }
  }
}
