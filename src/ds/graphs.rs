//! This submodule provides a basic graph framework.

use std::collections::{BinaryHeap, HashSet};

use crate::helper;
use crate::io;
use std::cmp;

/// type of graph
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GraphType {
    DIRECTED,
    UNDIRECTED,
}

///
/// this trait defines the essential behaviour
/// of a graph
///
pub trait Graph {
    fn add_edge(&mut self, v1: usize, v2: usize);

    fn get_adjacent_vertices(&self, v: usize) -> Vec<usize>;

    fn get_spanning_tree(&self, v: usize) -> Box<dyn Graph>;

    fn size(&self) -> usize;

    fn display(&self);
}

///
/// graph representation using an adjacency set
///
pub struct AdjacencySet {
    vertices: Vec<Vertex>,
    n: usize,
    kind: GraphType,
}

impl AdjacencySet {
    /// create a new adjacency set based graph
    pub fn new(n: usize, kind: GraphType) -> Self {
        let mut vs = Vec::new();
        for i in 0..n {
            vs.push(Vertex::new(i));
        }

        AdjacencySet {
            vertices: vs,
            n: n,
            kind: kind,
        }
    }

    pub fn dummy() -> Self {
        AdjacencySet {
            vertices: Vec::new(),
            n: 0,
            kind: GraphType::UNDIRECTED,
        }
    }
}

impl Graph for AdjacencySet {
    fn add_edge(&mut self, v1: usize, v2: usize) {
        if v1 >= self.n || v2 >= self.n {
            panic!("add_edge: invalid vertex or vertices, {} to {}", v1, v2);
        }

        self.vertices[v1].vs.insert(v2);
        if self.kind == GraphType::UNDIRECTED {
            self.vertices[v2].vs.insert(v1);
        }
    }

    /// retrieve the adjacent vertices of the given vertex
    fn get_adjacent_vertices(&self, v: usize) -> Vec<usize> {
        if v >= self.n {
            panic!("get_adjacent_vertices: invalid vertex {}", v);
        }

        let mut vs = Vec::new();
        for i in 0..self.n {
            if self.vertices[v].vs.contains(&i) {
                vs.push(i);
            }
        }

        vs.sort();

        vs
    }

    ///
    /// Get a spanning tree of the given graph. This is a slight variation of
    /// Prim's Algorithm and generates a randomised spanning tree.
    ///
    /// This is especially evident on larger mazes where the generated spanning tree
    /// produces visibly distinct configurations instead of the same one each time.
    ///
    fn get_spanning_tree(&self, source: usize) -> Box<dyn Graph> {
        if source > self.n {
            panic!("get_spanning_tree: invalid vertex {}", source);
        }

        if self.kind == GraphType::DIRECTED {
            io::print_message_and_quit("spanning tree not defined for directed graphs");
        }

        let mut visited = HashSet::new();
        visited.insert(source);

        let mut priority_queue = BinaryHeap::new();

        for neighbour in self.get_adjacent_vertices(source) {
            if !visited.contains(&neighbour) {
                priority_queue.push(Edge::new(source, neighbour));
            }
        }

        let mut spanning_tree = AdjacencySet::new(self.size(), self.kind);
        while !priority_queue.is_empty() {
            let edge = priority_queue.pop().unwrap();

            if visited.contains(&edge.to) {
                continue;
            }

            visited.insert(edge.to);
            for next_neighbour in self.get_adjacent_vertices(edge.to) {
                if !visited.contains(&next_neighbour) {
                    priority_queue.push(Edge::new(edge.to, next_neighbour));
                }
            }
            spanning_tree.add_edge(edge.from, edge.to);
        }

        Box::new(spanning_tree)
    }

    /// the number of vertices of the graph
    fn size(&self) -> usize {
        self.n
    }

    fn display(&self) {
        for vertex in 0..self.n {
            print!("{} : ", vertex);
            for neighbour in self.get_adjacent_vertices(vertex) {
                print!("{} ", neighbour);
            }
            println!();
        }
    }
}

///
/// the vertex type for use by AdjacencySet
///
#[derive(Debug)]
struct Vertex {
    // the id of this vertex
    v: usize,
    // the neighbours of this vertex
    vs: HashSet<usize>,
}

impl Vertex {
    fn new(v: usize) -> Self {
        Vertex {
            v: v,
            vs: HashSet::new(),
        }
    }
}

///
/// Custom Edge type for Priority Queue used for generaring the spanning tree
///
struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    fn new(from: usize, to: usize) -> Self {
        Edge { from, to }
    }
}

///
/// impl a bunch of traitS so that the `Edge` type can be used in the priority queue used by Prim's
/// Algorithm to generate the spanning tree. This is because standard Rust does not provide a way
/// to pass in a custom comparator when creating a `binary_heap` instance (which is the equivalent
/// of Java's PriorityQueue, albeit a max heap instead of a min heap).
///
impl cmp::PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl cmp::Eq for Edge {}

impl cmp::PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Edge {
    fn cmp(&self, _other: &Edge) -> cmp::Ordering {
        if helper::get_random_number_in_range(0, 1) == 0 {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// tests that produce output on standard output need to be run with
    /// the --nocapture argument passed in, as in:
    ///
    /// `$cargo test -- --nocapture
    /// `
    ///
    #[test]
    fn test_generate_spanning_tree() {
        let mut g = AdjacencySet::new(5, GraphType::UNDIRECTED);

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 4);
        g.add_edge(4, 1);
        g.add_edge(4, 3);

        let spanning_tree = g.get_spanning_tree(0);

        spanning_tree.display();
    }

    #[test]
    fn test_spanning_tree_params() {
        let mut g = AdjacencySet::new(5, GraphType::UNDIRECTED);

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        g.add_edge(2, 4);

        let spanning_tree = g.get_spanning_tree(0);
        assert_eq!(spanning_tree.size(), g.size());
    }

    #[test]
    #[should_panic]
    fn test_add_edge_panic() {
        let mut g = AdjacencySet::new(1, GraphType::DIRECTED);

        g.add_edge(0, 1);
    }

    #[test]
    #[should_panic]
    fn test_get_adjancent_vertices_panic() {
        let mut g = AdjacencySet::new(2, GraphType::UNDIRECTED);

        g.add_edge(0, 1);

        let _ = g.get_adjacent_vertices(2);
    }

    #[test]
    #[should_panic]
    fn test_get_spanning_tree_panic() {
        let mut g = AdjacencySet::new(5, GraphType::UNDIRECTED);

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(2, 4);
        g.add_edge(4, 1);

        let _ = g.get_spanning_tree(10);
    }
}
