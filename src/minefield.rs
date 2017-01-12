use tile;

pub type Minefield = [[tile::Tile; MAP_SIZE_WIDTH]; MAP_SIZE_HEIGHT];

// Assemble a minefield
pub const MAP_SIZE_WIDTH: usize = 60;
pub const MAP_SIZE_HEIGHT: usize = 30;
