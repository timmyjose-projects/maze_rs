//! This module contains the essential data structures used by the Maze project
//! The basic graph types are contained in a submodule, `graphs`.

pub mod graphs;

///
/// A two-dimensional point representing the location of a cell of the maze
/// on the screen
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    // getters
    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}

///
/// Conceptual representation of the direction change
/// between cells along a path, not of the cell itself
///
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

///
/// Represents a cell of the maze. `id` maps to the vertex id of the
/// equivalent undirected graph, and `location` simply refers to the
/// coordinates of this cell on the screen, for the purposes of
/// rendering the cell.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CellData {
    id: usize,
    location: Point,
}

impl CellData {
    pub fn new(id: usize, location: Point) -> Self {
        CellData { id, location }
    }

    // getters
    pub fn get_location(&self) -> &Point {
        &self.location
    }
}

///
/// This represents the amze itself, for the purposes of rendering the
/// maze on the screen. The individuals cells of this maze struct are
/// conceptual representations of the cells of the maze on the screen.
///
#[derive(Debug)]
pub struct MazeData {
    height: usize,
    width: usize,
    cells: Vec<Vec<CellData>>,
}

impl MazeData {
    pub fn new(height: usize, width: usize, cells: Vec<Vec<CellData>>) -> Self {
        MazeData {
            height,
            width,
            cells,
        }
    }

    // getters
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    // for the given coordinates, return the actual cell metadata
    // that this MazeData instance holds
    pub fn get_cell(&self, x: usize, y: usize) -> &CellData {
        &self.cells[x][y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p = Point::new(1, 2);

        assert_eq!(p, Point { x: 1, y: 2 });
    }

    #[test]
    fn teste_maze_data_sanity() {
        let maze_data = MazeData::new(10, 20, Vec::new());

        assert_eq!(maze_data.get_height(), 10);
        assert_eq!(maze_data.get_width(), 20);
    }

    #[test]
    fn test_maze_data() {
        let maze_data = MazeData::new(
            2,
            3,
            vec![
                vec![
                    CellData::new(0, Point::new(0, 0)),
                    CellData::new(1, Point::new(0, 1)),
                    CellData::new(2, Point::new(0, 2)),
                ],
                vec![
                    CellData::new(3, Point::new(1, 0)),
                    CellData::new(4, Point::new(1, 1)),
                    CellData::new(5, Point::new(1, 2)),
                ],
            ],
        );

        assert_eq!(
            maze_data.get_cell(1, 1),
            &CellData::new(4, Point::new(1, 1))
        );
        assert_eq!(
            maze_data.get_cell(0, 2),
            &CellData::new(2, Point::new(0, 2))
        );
    }
}
