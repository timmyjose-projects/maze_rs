# Approach


The basic approach to actually generating and solving the maze is straightforward - treat the rectangular
grid as a connected, undirected graph in which the vertices represent the "cells" of the maze, and the 
edges represent the "walls" of the maze. 

The maze is therefore represented in two distinct ways - one as a normal matrix with rectangular cells for the 
purposes of rendering on the screen, and conceptually as a graph upon which we can then apply the usual graph
algorithms - BFS, DFS, Prim's et al.

The main idea to get the "perfect maze" is to simply generate a Spanning Tree for the maze. This ensures that
every cell in the maze is reachable from every other cell of the same maze, as well as ensuring that a unique
path exists between a unique pair of cells.

Solving the maze then simply entails implementing variations of the quintessential graph algorithms - DFS and BFS
to solve for the various desired configurations.

The main task/challenge is to actually render the maze on the terminal screen itself. If the task were to simply 
solve the maze as we generate it, it would have been much simpler to simply use a recursive backtracking algorithm
(such as randomised DFS, for instance). However, I decided against that since I wanted some level of dynamic
capabilities - a crude form of animation as well as the freedom to render the cells in no particular order. This,
of course, brings about its own set of limitations, which I feel are justifiable enough for the scope of this project.

More specific design details follow in the individual sections.


## Generating the Maze

Generating the maze has three essential steps:

  1. Reading in the dimensions from the user and generating a two dimensional matrix of conceptual "cells".
    Each cell will basically hold metadata about the conceptual maze - coordinates for rendering, as well as
    an id so that it can be mapped to the corresponding conceptual graph (see point 2).

  2. Mapping the cells and the walls between the cells to a corresponding connected, undirected graph's 
    vertices and edges.

  3. Generating a Spanning Tree using Prim's Algorithm (a randomised version to ensure variety in the shapes of the
    generated mazes between runs). Since this maze is essentially a tree, simple DFS would have sufficed, but I find
    Prim's to be a much simpler algorithm.

  4. Finally, rendering the Spanning Tree state onto the screen using ANSI escape codes (see the next subsection for 
     more details on this).  	    



### ANSI Escape Codes and rendering the Maze

As mentioned before, the basic approach to rendering the maze (in any state) is to use ANSI escape codes (see the
[#References] section below. This means that the code should essentially work for any terminal that has basic VT100
(or better) support. Note that this also means that it will not work on Windows' default terminals. It should work 
on a terminal emulator like Cygwin/ConEmu/Cmder/Git Bash etc. though.

Note also that due to the variety of available dimensions of terminal screen configurations, I have decided to allow
only for a maximum of 80x24 (80 columns and 24 rows), which should work on almost any terminal configuration. 

That also means that we allow only for a (hardcoded) limit of a 25 x 25 maze (even though on my MBP at high resolution, 
at least 116x65 seems to be supported). Given enough time experimenting witn scrolling options using ANSI Escape Codes,
this limit can easily be rescinded. 


We use ANSI Escape Codes specifically for the following purposes:

  1. Clearing the terminal and setting the cursor at the top-left of the screen.

  2. Locating a particular (LINE, COLUMN) location on the screen for rendering a conceptual cell as part of a maze.
     To this end, each maze cell is considered to consist of the following "sprites":

      `+---+` for the North and South walls, and `|` for the East and West walls

     with the net effect that a cell (before modification) would look so:
     
        +---+
        |   |
        +---+
    
      and the entire maze is built up using this basic structure.

  3. Given the Spanning Tree (ST) state, a simple DFS will provide the path from any particular vertex to any other
     vertex in the ST, and this path will be unique. The rendering of a "solved" state of the maze simply involves
     rendering only the cells/vertices in this path by locating the position of the cell on the actual terminal screen
     (using the coordinates stored in the cell metadata), and rendering the appropriate character.

     The character to be displayed within a cell is determined from the direction of the movement. For a starting cell, 
     it is 's', for a terminal cell it is 't', and for an intermediate cell in the path, the proper character, '>', 'v',
     '<', or '^' is chosen by simply determing the relative direction changes between two consecutive cells along the 
     path.

     For instance, Suppose we have a path like so: (0, 0) -> (0, 1) -> (1, 1) -> (2, 1). The translated path for this 
     would be: s v v t. This is because we can only move one step in the horizontal or vertical direction at a time.        

     Also note that the coordinate system used here is basically the same as that for SVG et al - the origin point is the 
     top-left of the screen, and it grows downwards in either direction. For the ANSI Escape Codes in particular, an (x, y)
     coordinate represents LINE x, COLUMN y from the origin.



## Solving the Maze

To solve the maze in this context means finding the path from the top-left cell to the bottom-right cell of the maze.

Given the Spanning Tree (which now represents the conceptual maze), we simply perform DFS from cell 0 (proper coordinates
(0, 0) all the way down to cell height x width - 1 (proper coordinates (height-1, width-1)). As noted before, this path is
unique. The starting point is marked with 's' and the ending point is marked with 't', and the overall path is rendered as
explained in the previous section.



## Solving for the Longest Path

For the longest path problem, we use the common "trick" of finding the longest path between two vertices of a tree - using
two BFSes. 

The first BFS (starting from any vertex is fine, but we can always start from cell 0 for consistency) will ensure that the 
last vertex in the returned path is one end of *a* longest path (there can be many more, of course). Then, if we perform 
another BFS from this vertex, we will get another longest path (which maybe the same as before) which starts with this 
vertex. 

As before, render the whole path using ANSI Escape codes.



## Solving for the Most Convoluted Path

**OUT OF SCOPE**. (One approach that is feasible is to find the path between each pair of vertices, and calculate "complexity" by, say, calculating how
many direction changes (consecutively) are present in the path, and then rendering the path with the maximum value. If the grid size is
`N x M`, then there are N * M vertices, and since we pick 2 of them as the start and end points at a time, there should be a maximum of 
NMC2 (NxM choose 2) iterations. For the maximum grid size of 25 x 25, that would be around 19500 iterations, which seems reasonable enough).

Scoping this out for this release due to time constraints.



## References

Some references that I found useful dealing with 

* [http://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html](http://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html)

* [https://en.wikipedia.org/wiki/ANSI_escape_code](https://en.wikipedia.org/wiki/ANSI_escape_code)




