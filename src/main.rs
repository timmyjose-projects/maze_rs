extern crate maze_rs;

use maze_rs::core;
use maze_rs::helper;
use maze_rs::io;

static USAGE: &'static str = "Usage: cargo run HEIGHT WIDTH";
static MENU: &'static str = "\nEnter choice (1 - solve, 2 - longest path, 3 - quit)... \n";

///
/// The entry-point for the maze project.
///
fn main() {
    let args = io::get_args();

    if args.len() != 2 {
        io::print_message_and_quit(USAGE);
    }

    // validate the supplied dimensions
    match helper::get_maze_dimensions(&args) {
        Ok((h, w)) => {
            // generate maze with the given dimensions
            let mut maze = core::Maze::initialize_maze(h, w);
            maze.create_maze();

            loop {
                io::print_message(MENU);

                if let Some(option) = io::get_number() {
                    match option {
                        1 => maze.solve_maze(),
                        2 => maze.print_longest_path(),
                        3 => io::print_message_and_quit("Goodbye!\n"),
                        _ => continue,
                    }
                }
                // avoid scrolling down the menu
                io::adjust_menu_location_on_screen();
            }
        }
        Err(e) => io::print_error_and_quit(Box::new(e)),
    }
}
