//! This module defines functions that actually render the maze by rendering
//! each cell one at a time. Each cell, in turn, is rendered by locating the
//! appropriate location on the screen, and rendering the individual sprites
//! forming that cell.

///
/// private helper functions that trigger the ANSI Escape Codes
///

/// clear the screen and place the cursor at the origin (top-left of
/// the screen

use super::*;
use super::super::ds::{CellData, Direction, MazeData};
use super::super::io::flush;

/// clear the terminal screen
fn cls() {
    print!("{}", CLS);
}

/// line feed
fn newline() {
    println!();
}

/// go to a specific location on the screen
fn locate(x: usize, y: usize) {
    print!("{}", format!("\x1B[{};{}H", x + 1, y + 1));
}

/// save the current position of the cursor on the screen -
/// this is saved in the terminal's buffer
fn save_cursor_position() {
    print!("{}", SAVE_CURSOR_POSITION);
}

/// restore the cursor to the last saved cursor location
fn restore_cursor_position() {
    print!("{}", RESTORE_CURSOR_POSITION);
}

/// change the font colour to the provided
/// colour, falling back on defaults in case
/// colours are not supported by the terminal
fn set_cursor_color(color: Color) {
    print!("{}", color.as_str());
}

/// reset the font colour to the original
/// colour before `set_cursor_color` was
/// called last
fn reset_color() {
    print!("{}", RESET_COLOR);
}

/// pause the animation for the given
/// duration (in milliseconds)
fn pause(duration: u64) {
    use std::{thread, time};

    thread::sleep(time::Duration::from_millis(duration));
    flush();
}

///
/// The public API
///

/// Render the given maze on the screen
/// using ANSI Escape Codes
pub fn draw_maze(maze: &MazeData) {
    cls();

    // go to the top-left of
    // the screen
    locate(0, 0);

    for i in 0..maze.get_height() {
        for j in 0..maze.get_width() {
            pause(MAZE_ANIMATION_SPEED);
            draw_cell(maze.get_cell(i, j));
        }
    }

    newline();
}

/// Render a single cell on the screen
/// by rendering the sprites corresponding
/// to the four walls of the cell.
pub fn draw_cell(cell: &CellData) {
    let (x, y) = (cell.get_location().get_x(), cell.get_location().get_y());

    locate(x, y);
    print!("{}", NORTH_SPRITE);

    locate(x + 1, y);
    print!("{}", WEST_SPRITE);

    locate(x + 1, y + 4);
    print!("{}", EAST_SPRITE);

    locate(x + 2, y);
    print!("{}", SOUTH_SPRITE);
}

/// adjust the menu location by erasing lines
/// and moving cursor back
pub fn delete_current_line() {
    for _ in 0..3 {
        print!("{}", CLEAR_CURRENT_LINE);
        print!("{}", MOVE_BACK_ONE_LINE);
    }
}

/// Erase the appropriate wall of the given cell by using
/// the direction information - this is simply for rendering
/// and does not actually mutate any state.
pub fn erase_wall(cell: &CellData, direction: &Direction) {
    save_cursor_position();

    let (x, y) = (cell.get_location().get_x(), cell.get_location().get_y());

    match direction {
        Direction::North => {
            locate(x, y + 1);
            for _ in 0..NORTH_SPRITE.len() - 2 {
                print!("{}", ' ');
            }
        }

        Direction::South => {
            locate(x + 2, y + 1);
            for _ in 0..SOUTH_SPRITE.len() - 2 {
                print!("{}", ' ');
            }
        }

        Direction::East => {
            locate(x + 1, y + 4);
            print!("{}", ' ');
        }

        Direction::West => {
            locate(x + 1, y);
            print!("{}", ' ');
        }
    }

    restore_cursor_position();
}

/// fill the given cell with the appropriate
/// direction character as part of animating
/// the path through the maze
pub fn fill_cell(cell: &CellData, c: char) {
    save_cursor_position();

    pause(PATH_ANIMATION_SPEED);
    set_cursor_color(Color::RED);

    let (x, y) = (cell.get_location().get_x(), cell.get_location().get_y());

    locate(x + 1, y + 2);
    print!("{}", c);

    reset_color();
    restore_cursor_position();
}

/// clear the contents of the current cell
/// so that the maze can be animated again
pub fn clear_cell(cell: &CellData) {
    save_cursor_position();

    let (x, y) = (cell.get_location().get_x(), cell.get_location().get_y());

    locate(x + 1, y + 2);
    print!("{}", ' ');

    restore_cursor_position();
}
