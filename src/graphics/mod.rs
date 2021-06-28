//! This module handles all the screen rendering functionality used both for
//! generating the maze, as well as solving it.

pub mod renderer;

///
/// ANSI Escaoe Codes
///

/// clear screen
pub static CLS: &'static str = "\x1B[2J";

/// erase the current line
pub static CLEAR_CURRENT_LINE: &'static str = "\x1B[2K";

/// move cursor one position back
pub static MOVE_BACK_ONE_LINE: &'static str = "\x1B[F";

/// save the cursor's current location in the terminal's buffer
pub static SAVE_CURSOR_POSITION: &'static str = "\x1B[s";

/// restore the cursor to the last saved cursor location
pub static RESTORE_CURSOR_POSITION: &'static str = "\x1B[u";

/// reset font colour
pub static RESET_COLOR: &'static str = "\x1B[0m";

///
/// constants used for rendering the cells
///

/// initial location offset of the column cursor (from origin)
pub const COL_INIT: usize = 3;
/// column increment for cell sprite
pub const COL_OFFSET: usize = 3;

/// initial location offset of the column cursor (from origin)
pub const LINE_INIT: usize = 2;
/// line increment for cell sprite
pub const LINE_OFFSET: usize = 1;

/// rendering (pause) speed of the cells
pub const MAZE_ANIMATION_SPEED: u64 = 2; // ms
/// rendering (pause) speed of the path through the maze
pub const PATH_ANIMATION_SPEED: u64 = 150; // ms

/// Sprites for rendering a cell in the maze
pub static NORTH_SPRITE: &'static str = "+---+";
pub static EAST_SPRITE: &'static str = "|";
pub static SOUTH_SPRITE: &'static str = "+---+";
pub static WEST_SPRITE: &'static str = "|";

/// constants for font colours
static RED_FONT: &'static str = "\x1B[31m";
static GREEN_FONT: &'static str = "\x1B[32m";
static BLUE_FONT: &'static str = "\x1B[34m";

///
/// Enum representing various colours that may be
/// used for rendering a path through the maze.
/// If the terminal does not support these
/// colours, then the terminal will fall back to
/// its default colours.
///
#[derive(Debug)]
pub enum Color {
    RED,
    GREEN,
    BLUE,
}

impl Color {
    fn as_str(&self) -> &'static str {
        match *self {
            Color::RED => RED_FONT,
            Color::GREEN => GREEN_FONT,
            Color::BLUE => BLUE_FONT,
        }
    }
}
