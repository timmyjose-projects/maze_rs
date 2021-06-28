//! This module contains common helper functions.

// import the `rand` crate since Rust's standard library does not have
// support for random number generation

use rand::prelude::*;

use crate::error::*;
use std::str::FromStr;

use crate::ds::{CellData, Direction};

/// get the maze's height and width from the command-line arguments
/// with suitable validation
pub fn get_maze_dimensions(args: &Vec<String>) -> Result<(usize, usize)> {
    if let Ok(h) = usize::from_str(args[0].trim()) {
        if let Ok(w) = usize::from_str(args[1].trim()) {
            if h < 1 || w < 1 {
                return Err(MazeError::of(ErrorKind::InvalidDimensions));
            }
            return Ok((h, w));
        }
    }
    Err(MazeError::of(ErrorKind::InvalidDimensionsNotNumber))
}

/// Return a random number in the closed range [l, h]
/// This uses the `rand` crate to generate PRNs.
pub fn get_random_number_in_range(l: isize, h: isize) -> isize {
    let mut rng = thread_rng();

    rng.gen_range(l..h + 1)
}

/// find the relative direction of the source cell w.r.t the neighbouring
/// cell by using their coordindates
pub fn get_direction(from: &CellData, to: &CellData) -> Direction {
    let fx = from.get_location().get_x();
    let fy = from.get_location().get_y();
    let tx = to.get_location().get_x();
    let ty = to.get_location().get_y();

    if fx < tx {
        return Direction::South;
    }

    if fx > tx {
        return Direction::North;
    }

    if fy < ty {
        return Direction::East;
    }

    Direction::West
}

/// map the direction to the appropriate character for rendering
pub fn get_char_for_direction(direction: &Direction) -> char {
    match direction {
        Direction::North => '^',
        Direction::South => 'v',
        Direction::East => '>',
        Direction::West => '<',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds::Point;

    #[test]
    fn test_get_direction_south() {
        let from = CellData::new(0, Point::new(1, 2));
        let to = CellData::new(1, Point::new(2, 2));

        assert_eq!(get_direction(&from, &to), Direction::South);
    }

    #[test]
    fn test_get_direction_west() {
        let from = CellData::new(0, Point::new(1, 2));
        let to = CellData::new(1, Point::new(1, 1));

        assert_eq!(get_direction(&from, &to), Direction::West);
    }

    #[test]
    fn test_get_direction_east() {
        let from = CellData::new(0, Point::new(1, 2));
        let to = CellData::new(1, Point::new(1, 3));

        assert_eq!(get_direction(&from, &to), Direction::East);
    }

    #[test]
    fn test_get_direction_north() {
        let from = CellData::new(0, Point::new(1, 2));
        let to = CellData::new(1, Point::new(0, 2));

        assert_eq!(get_direction(&from, &to), Direction::North);
    }

    #[test]
    fn test_char_for_direction_east() {
        let direction = Direction::East;

        assert_eq!(get_char_for_direction(&direction), '>');
    }

    #[test]
    fn test_char_for_direction_west() {
        let direction = Direction::West;

        assert_eq!(get_char_for_direction(&direction), '<');
    }

    #[test]
    fn test_char_for_direction_north() {
        let direction = Direction::North;

        assert_eq!(get_char_for_direction(&direction), '^');
    }

    #[test]
    fn test_char_for_direction_southt() {
        let direction = Direction::South;

        assert_eq!(get_char_for_direction(&direction), 'v');
    }
}
