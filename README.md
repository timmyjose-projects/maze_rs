# Maze Project

The aim is to be able to generate perfect mazes - mazes that are always solve-able for any pair of
cells in the maze with the additional property that there exists a unique solution for that pair.
In addition, solve the maze for the given scenarios:

  * Chart the path from the top-left to the bottom-right of the maze.

  * Discover *a* longest path in the maze. 

  * Discover *a* most convoluted path in the maze.


The project will generate randomly shaped mazes for every run of the program (within reason).

For the design of the project, please refer to the [Design doc](https://github.com/timmyjose/maze_rs/blob/master/doc/Design.md).



## Project Setup

This project is written in Rust (version: rustc 1.28.0 (9634041f0 2018-07-30)). Rust uses `cargo` as the build/dependency manager, and `rustup` as the tool for installing
Rust toolchains.

To set up Rust, please follow the following steps:

  1. Go to the [rustup.rs](https://rustup.rs/) site to install the latest version of `rustup` using the command shown on the site:
       `curl https://sh.rustup.rs -sSf | sh`

  2. Following the default options should install the toolchain management tool, `rustup`. Note that you can pick from amongst a 
     number of preset toolchains - `stable`, `nightly`, and some others. Choose `stable` (`nightly` should work fine as well though).
  
  3. Verify that the installation went through fine:
  		```
  		$ cargo --version
		cargo 1.28.0 (96a2c7d16 2018-07-13)
		```


### Building the project

Note: This project should work both with all Rust editions - `2015` or `2018`.

From the project root, run the following command: `cargo clean && cargo build --release` as in:

```
$ cargo clean && cargo build --release
   Compiling libc v0.2.43
   Compiling rand_core v0.2.1
   Compiling rand v0.5.5
   Compiling maze_project v0.1.0 (file:///Users/z0ltan/Code/Projects/games/maze_rs)
    Finished release [optimized] target(s) in 4.59s

```

### Generating documentation for the Project

Rust comes with its own documentation tool (much like `javadoc`) built-in. To generate the project documentation, run
the following command: `cargo doc --no-deps --open` as in:

```
$ cargo doc --no-deps --open
    Checking rand_core v0.2.1
    Checking libc v0.2.43
    Checking rand v0.5.5
 Documenting maze_project v0.1.0 (file:///Users/z0ltan/Code/Projects/games//maze_rs)
    Finished dev [unoptimized + debuginfo] target(s) in 4.35s
     Opening /Users/z0ltan/Code/Projects/games/maze_rs/target/doc/maze_rs/index.html
   Launching open
```

This should open the generated HTML page in your default browser.


### Running the project

To run the project, use the `cargo run` command with any parameters passed in, like so:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/maze_project`
Usage: cargo run HEIGHT WIDTH
```


### Running the tests

Rust comes with its own test suite built-in, and the `cargo test` command should run all tests in the project:

```
$ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running target/debug/deps/maze_project-c360a1a0a2cd554c

running 16 tests
test ds::tests::test_maze_data ... ok
test ds::tests::test_point ... ok
test ds::tests::teste_maze_data_sanity ... ok
test helper::tests::test_char_for_direction_east ... ok
test helper::tests::test_char_for_direction_north ... ok
test helper::tests::test_char_for_direction_southt ... ok
test ds::graphs::test::test_add_edge_panic ... ok
test ds::graphs::test::test_get_adjancent_vertices_panic ... ok
test ds::graphs::test::test_get_spanning_tree_panic ... ok
test helper::tests::test_char_for_direction_west ... ok
test helper::tests::test_get_direction_east ... ok
test helper::tests::test_get_direction_north ... ok
test helper::tests::test_get_direction_south ... ok
test helper::tests::test_get_direction_west ... ok
test ds::graphs::test::test_spanning_tree_params ... ok
test ds::graphs::test::test_generate_spanning_tree ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/maze_project-c14ba56a651feb63

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests maze_project

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

## Sample Run

Simply run the project with the desired dimensions of the grid. Shown below is a sample run:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/maze_project`
Usage: cargo run HEIGHT WIDTH


$ cargo run 15 20
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/maze_project 15 20`


   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
   | s                             |                           |   |   |   |   |   |
   +   +   +   +---+   +   +---+---+   +---+   +---+---+   +---+   +   +   +   +   +
   | v |   |       |   |           |       |           |                           |
   +   +---+---+---+---+   +   +   +   +   +---+---+   +---+---+---+---+---+---+---+
   | v     |       |   |   |   |       |           |               |       |       |
   +   +   +   +   +   +---+   +---+   +---+---+---+   +---+---+---+---+   +   +   +
   | v |       |           |   |   |               |   |   |       |   |   |   |   |
   +   +---+---+---+---+---+   +   +---+---+---+---+---+   +---+   +   +   +   +---+
   | >   v             |   |   |                       |       |   |   |           |
   +   +   +   +   +---+   +---+   +   +---+---+---+---+   +---+   +   +   +   +---+
   |   | v |   |                   |               |       |   |           |       |
   +   +   +   +---+   +   +   +---+   +   +   +---+---+   +   +   +   +---+---+   +
   |   | v |   |       |   |       |   |   |       |   |       |   |       |       |
   +   +   +---+---+---+   +---+   +   +---+---+---+   +   +---+   +---+---+---+   +
   |   | v             |       |   |   |           |               |   |   |   |   |
   +---+   +---+   +---+---+---+---+---+---+   +---+---+   +   +---+   +   +   +---+
   |     v     |                       |       |   |       |   |                   |
   +   +   +---+---+   +   +---+---+---+   +---+   +   +   +---+   +---+---+---+   +
   |   | v         |   |           |       |   |       |           |           |   |
   +   +   +---+---+   +---+---+---+---+   +   +   +   +---+---+---+   +---+   +---+
   |   | >   v     |           |   |       |   |   |       |               |   |   |
   +---+   +   +---+---+---+---+   +---+   +   +   +---+---+   +   +---+---+---+   +
   |       | >   v         |   |       |   |       |   |       |                   |
   +   +---+   +   +---+---+   +   +---+   +   +---+   +---+   +---+---+---+---+---+
   |   |       | >   >   v     |           |       |   |       | >   >   >   >   v |
   +---+   +   +   +---+   +---+   +---+---+   +---+   +   +---+   +---+---+---+   +
   |       |   |   |   | >   >   >   >   >   >   >   >   >   >   ^             | v |
   +   +---+   +   +   +---+   +   +---+   +---+   +---+   +   +---+---+   +---+   +
   |       |   |           |   |   |           |       |   |           |       | t |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

Enter choice (1 - solve, 2 - longest path, 3 - quit)...
1

   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
   | s                             |                           |   |   |   |   |   |
   +   +   +   +---+   +   +---+---+   +---+   +---+---+   +---+   +   +   +   +   +
   | v |   |       |   |           |       |           |                           |
   +   +---+---+---+---+   +   +   +   +   +---+---+   +---+---+---+---+---+---+---+
   | v     |       |   |   |   |       |           |               |       | >   v |
   +   +   +   +   +   +---+   +---+   +---+---+---+   +---+---+---+---+   +   +   +
   | v |       |           |   |   |               |   |   |       |   |   | ^ | t |
   +   +---+---+---+---+---+   +   +---+---+---+---+---+   +---+   +   +   +   +---+
   | >   v             |   |   |                       |       |   |   | >   ^     |
   +   +   +   +   +---+   +---+   +   +---+---+---+---+   +---+   +   +   +   +---+
   |   | v |   |                   |               |       |   | >   >   ^ |       |
   +   +   +   +---+   +   +   +---+   +   +   +---+---+   +   +   +   +---+---+   +
   |   | v |   |       |   |       |   |   |       |   |       | ^ |       |       |
   +   +   +---+---+---+   +---+   +   +---+---+---+   +   +---+   +---+---+---+   +
   |   | v             |       |   |   |           |     >   >   ^ |   |   |   |   |
   +---+   +---+   +---+---+---+---+---+---+   +---+---+   +   +---+   +   +   +---+
   |     v     |                       |       |   | >   ^ |   |                   |
   +   +   +---+---+   +   +---+---+---+   +---+   +   +   +---+   +---+---+---+   +
   |   | v         |   |           |       |   | >   ^ |           |           |   |
   +   +   +---+---+   +---+---+---+---+   +   +   +   +---+---+---+   +---+   +---+
   |   | >   v     |           |   |       |   | ^ |       |               |   |   |
   +---+   +   +---+---+---+---+   +---+   +   +   +---+---+   +   +---+---+---+   +
   |       | >   v         |   |       |   | >   ^ |   |       |                   |
   +   +---+   +   +---+---+   +   +---+   +   +---+   +---+   +---+---+---+---+---+
   |   |       | >   >   v     |           | ^     |   |       |                   |
   +---+   +   +   +---+   +---+   +---+---+   +---+   +   +---+   +---+---+---+   +
   |       |   |   |   | >   >   >   >   >   ^                                 |   |
   +   +---+   +   +   +---+   +   +---+   +---+   +---+   +   +---+---+   +---+   +
   |       |   |           |   |   |           |       |   |           |       |   |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

Enter choice (1 - solve, 2 - longest path, 3 - quit)...
2

```



## Limitations and Possible Enhancements

  * Uses ANSI Escape Codes for rendering the maze, and so scrolling is a problem. Recommended limits are a 25 x 25 maze,
    but on bigger screens with greater resolutions and configurations, more or less should be possible. The code does not
    enforce any limit other than a minimum of a 1 x 1 maze size.

  * Again, due to the use of ANSI Escape Codes, the code should work fine on any ANSI-compliant terminal, and that rules out
    basic Windows command lines.

  * Most complicated Paths should be relatively straightforward to implement, and the planned approach is mentioned in the
    supporting [Design doc](https://github.com/timmyjose/maze_rs/blob/master/doc/Design.md).

  * All tweakable configurations (animation speeds, font colours, cell sprites et al) should ideally be placed in an external
    configuration file.  



## Licence

Please refer to the licence file at [LICENCE](https://github.com/timmyjose/maze_rs/blob/master/LICENSE.md)



