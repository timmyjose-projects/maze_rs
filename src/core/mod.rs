//! This module contains the core functionality of the project:
//! Maze Generation and Maze Solving algorithms.

use std::collections::{HashMap, VecDeque};

use crate::ds;
use crate::ds::graphs::{self, Graph};
use crate::graphics;
use crate::helper;

///
/// The actual maze itself - both in terms of data to be manipulated in the form
/// of graph algorithms, as well as all the data needed to actually render the cells
/// along paths on the terminal screen.
///
pub struct Maze {
    height: usize,
    width: usize,
    maze_data: ds::MazeData,
    spanning_tree: Box<dyn graphs::Graph>,
    maze_state: HashMap<usize, ds::CellData>,
    maze_solved: bool,
    maze_solved_path: Vec<usize>,
    longest_path_solved: bool,
    longest_path_solved_path: Vec<usize>,
}

impl Maze {
    /// Static method to  populate the cells of the MazeData structure by mapping
    /// the coordinates of the cells to their actual future locations
    /// on the screen.
    /// This returns a new maze instance which is used for all further
    /// operations.
    pub fn initialize_maze(height: usize, width: usize) -> Self {
        let mut cells: Vec<Vec<ds::CellData>> = Vec::new();
        for _ in 0..height {
            cells.push(Vec::new());
        }

        let mut x = graphics::LINE_INIT;
        let mut y = graphics::COL_INIT;

        for i in 0..height {
            for j in 0..width {
                cells[i].push(ds::CellData::new(
                    width * i + j,
                    ds::Point::new(i + x, j + y),
                ));
                y += graphics::COL_OFFSET;
            }
            // reset the y component - note that in this coordinate system, the Y-axis is along the
            // horizontal line from the origin, and the X-Axis is the vertical line from the origin
            y = graphics::COL_INIT;
            x += graphics::LINE_OFFSET;
        }

        let mut new_maze = Maze {
            height: height,
            width: width,
            maze_data: ds::MazeData::new(height, width, cells),
            spanning_tree: Box::new(graphs::AdjacencySet::dummy()),
            maze_state: HashMap::new(),
            maze_solved: false,
            maze_solved_path: Vec::new(),
            longest_path_solved: false,
            longest_path_solved_path: Vec::new(),
        };

        // Map from the coordinates of each cell to the corresponding
        // CellData structure
        for i in 0..new_maze.height {
            for j in 0..new_maze.width {
                new_maze
                    .maze_state
                    .insert(new_maze.width * i + j, *new_maze.maze_data.get_cell(i, j));
            }
        }

        // return the newly created maze instance
        new_maze
    }

    ///
    /// Run the spanning tree algorithm, and generate a brand new maze from the grid
    /// created through initialize_maze
    ///<
    pub fn create_maze(&mut self) {
        let mut graph =
            graphs::AdjacencySet::new(self.height * self.width, graphs::GraphType::UNDIRECTED);

        // add horizontal connections between cells as edges
        for i in 0..self.height {
            for j in 0..self.width - 1 {
                graph.add_edge(self.width * i + j, self.width * i + j + 1);
            }
        }

        // add vertical connections between cells as edges
        for i in 0..self.height - 1 {
            for j in 0..self.width {
                graph.add_edge(self.width * i + j, self.width * i + j + self.width);
            }
        }

        // get the spanning tree starting from the top-left corner of the
        // maze
        self.spanning_tree = graph.get_spanning_tree(0);

        // display the maze as a set of cells with walls erased between the cells
        // forming part of the spanning tree
        self.draw_maze();

        let mut visited = vec![false; self.height * self.width];
        self.create_maze_helper(&mut visited, 0);
    }

    /// Helper function to erase the right walls in the current state of the
    /// maze using DFS
    fn create_maze_helper(&self, visited: &mut Vec<bool>, source_vertex: usize) {
        if visited[source_vertex] {
            return;
        }

        visited[source_vertex] = true;
        for neighbour in self.spanning_tree.get_adjacent_vertices(source_vertex) {
            let source_cell = self.maze_state.get(&source_vertex).unwrap();
            let neighbour_cell = self.maze_state.get(&neighbour).unwrap();

            let direction = helper::get_direction(&source_cell, &neighbour_cell);
            graphics::renderer::erase_wall(source_cell, &direction);
            self.create_maze_helper(visited, neighbour);
        }
    }

    /// Dispatch to the Renderer to display the current state of
    /// the maze (the MazeData instance)
    pub fn draw_maze(&self) {
        graphics::renderer::draw_maze(&self.maze_data);
    }

    ///
    /// Solve the current state of the maze. This simply uses DFS to plot the only path from
    /// the top-left corner of the maze to the bottom-right corner of the maze.
    ///
    pub fn solve_maze(&mut self) {
        self.prime_solved_states();

        let mut visited = vec![false; self.height * self.width];
        let mut path = Vec::new();

        let h = self.height;
        let w = self.width;

        self.solve_maze_helper(&mut visited, 0, h * w - 1, &mut path);
    }

    fn solve_maze_helper(
        &mut self,
        visited: &mut Vec<bool>,
        source: usize,
        target: usize,
        path: &mut Vec<usize>,
    ) {
        visited[source] = true;
        path.push(source);

        // if we have reached the target, then
        // process the path and return
        if source == target {
            self.render_path(&path);
            self.maze_solved = true;
            self.maze_solved_path = path.clone();
            return;
        }

        for neighbour in self.spanning_tree.get_adjacent_vertices(source) {
            if !visited[neighbour] {
                self.solve_maze_helper(visited, neighbour, target, path);
            }
        }

        // Didn't reach the target yet. Try the next vertex in line after
        // discarding the current vertex in the path.
        path.pop().unwrap();
        visited[source] = false;
    }

    /// render the given path by drawing each cell
    /// along the path of the given solution to
    /// the maze
    fn render_path(&self, path: &Vec<usize>) {
        // special handling for a single cell maze
        // and a two cell maze
        if path.len() == 1 {
            let cell = self.maze_state.get(&path[0]).unwrap();
            graphics::renderer::fill_cell(&cell, 's');
        } else if path.len() == 2 {
            let start_cell = self.maze_state.get(&path[0]).unwrap();
            let end_cell = self.maze_state.get(&path[1]).unwrap();
            graphics::renderer::fill_cell(&start_cell, 's');
            graphics::renderer::fill_cell(&end_cell, 't');
        } else {
            for i in 0..path.len() - 1 {
                let (curr_cell, next_cell) = (
                    self.maze_state.get(&path[i]).unwrap(),
                    self.maze_state.get(&path[i + 1]).unwrap(),
                );

                let direction = helper::get_direction(&curr_cell, &next_cell);

                // handle `source` and `target` cells, and handle the
                // general case separately
                if i == 0 {
                    graphics::renderer::fill_cell(&curr_cell, 's');
                } else if i == path.len() - 2 {
                    graphics::renderer::fill_cell(
                        &curr_cell,
                        helper::get_char_for_direction(&direction),
                    );
                    graphics::renderer::fill_cell(next_cell, 't');
                } else {
                    graphics::renderer::fill_cell(
                        &curr_cell,
                        helper::get_char_for_direction(&direction),
                    );
                }
            }
        }
    }

    /// Clear an already rendered path from the screen
    fn clear_path(&self, path: &Vec<usize>) {
        for i in 0..path.len() - 1 {
            let (curr_cell, next_cell) = (
                self.maze_state.get(&path[i]).unwrap(),
                self.maze_state.get(&path[i + 1]).unwrap(),
            );

            // handle `source` and `target` cells, and handle the
            // general case separately
            if i == 0 {
                graphics::renderer::clear_cell(&curr_cell);
            } else if i == path.len() - 2 {
                graphics::renderer::clear_cell(&curr_cell);
                graphics::renderer::clear_cell(&next_cell);
                break;
            } else {
                graphics::renderer::clear_cell(&curr_cell);
            }
        }
    }

    /// A mini State machine that ensures the progressions of
    /// Maze States as follows: Created -> [Solved | Longest Path | Most Complicated Path]
    fn prime_solved_states(&mut self) {
        if self.maze_solved {
            self.maze_solved = false;
            self.clear_path(&self.maze_solved_path);
        } else if self.longest_path_solved {
            self.longest_path_solved = false;
            self.clear_path(&self.longest_path_solved_path);
        }
    }

    ///
    /// Find a longest path in the maze using two BFS runs - the
    /// first run produces the endpoint of a longest path, and then
    /// the second run from this endpoint produces a longest path
    /// (which may be unique, of course).
    ///
    pub fn print_longest_path(&mut self) {
        self.prime_solved_states();

        // find a longest path from the source
        let first_run_path = self.find_longest_path(0);
        let longest_path_start = first_run_path[0];

        let longest_path = self.find_longest_path(longest_path_start);

        self.render_path(&longest_path);
        self.longest_path_solved = true;
        self.longest_path_solved_path = longest_path.clone();
    }

    fn find_longest_path(&self, start: usize) -> Vec<usize> {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        // distance table - the rows are the vertices of the spanning tree,
        // and the 0th column is the distance of this vertex, and the 1st
        // column is the last vertex to the current vertex.
        let mut distance_table = vec![vec![-1isize; 2]; self.spanning_tree.size()];

        distance_table[start][0] = 0;
        distance_table[start][1] = start as isize;

        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();

            for neighbour in self.spanning_tree.get_adjacent_vertices(vertex) {
                // if the neighbour has not been visited yet, update its distance
                // and last vertex info
                if distance_table[neighbour][0] == -1 {
                    distance_table[neighbour][0] = distance_table[vertex][0] + 1;
                    distance_table[neighbour][1] = vertex as isize;
                    queue.push_back(neighbour);
                }
            }
        }

        let mut farthest_vertex = 0;
        let mut max_distance = -1;
        for i in 0..self.spanning_tree.size() {
            if distance_table[i][0] > max_distance {
                max_distance = distance_table[i][0];
                farthest_vertex = i;
            }
        }

        // now chart the path by backtracking from
        // the farthest vertex to the starting vertex
        let mut longest_path = Vec::new();

        let mut stack = VecDeque::new();
        stack.push_front(farthest_vertex);

        let mut last_vertex = farthest_vertex;
        while last_vertex != start {
            stack.push_front(last_vertex);
            last_vertex = distance_table[last_vertex][1] as usize;
        }

        stack.push_front(start);

        while !stack.is_empty() {
            longest_path.push(stack.pop_front().unwrap());
        }

        longest_path
    }

    ///
    /// getters for maze properties
    ///
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
}
