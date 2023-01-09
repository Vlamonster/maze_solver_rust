# How to Run

```
Usage: maze_solver.exe [OPTIONS] <ROWS> <COLUMNS>                                                                                               
                                                                                                                                                
Arguments:                                                                                                                                      
  <ROWS>     Number of rows to draw                                                                                                             
  <COLUMNS>  Number of columns to draw                                                                                                          
                                                                                                                                                
Options:                                                                                                                                        
  -g, --generator <GENERATOR>  Generator used [default: depth_first_search] [possible values: depth_first_search, breadth_first_search, kruskal]
  -d, --delay <DELAY>          Number of milliseconds between animation [default: 25]                                                           
  -h, --help                   Print help information                                                                                           
  -V, --version                Print version information
```

Here are some examples:

```
# Generate 16 by 48 maze using the depth first seach generator and the default delay of 25ms.
cargo run --release -- 16 48 -g depth_first_search

# Generate 16 by 48 maze using the depth first seach generator and a delay of 0ms (instant).
cargo run --release -- 16 48 -g depth_first_search -d 0
```

I tested that this works on at least Windows 10, Ubuntu and macOS.

# Generators

The following generators are included:
<details><summary>Randomized depth-first search.</summary>

![](examples/dfs.gif)
</details>

<details><summary>Randomized breadth-first search.</summary>

![](examples/bfs.gif)
</details>

<details><summary>Kruskal's algorithm.</summary>

![](examples/kruskal.gif)
</details>

# Note on Design

It was important to me that large mazes could be drawn in a limited space, which meant that some thought had to be given
on how the maze should be represented. A simple (yet effective) representation would look like this:

```
+ +-+   [N, H]     
| | |   [V, V, V]     
+ +-+   [N, H]
|   |   [V, N, V]
+-+ +   [H, N]

H: Horizontal,
V: Vertical,
N: None,
```

This is 5x5 character matrix to represent a 2x2 maze. We can do better by combining the vertical and horizontal walls
into the same line:

```
_ ___   [H, N, H, H, H]
| |_|   [V, N, H, V, H]
|__ |   [V, H, N, N, V]

H: Horizontal,
V: Vertical,
N: None,
```

This gives us a 3x3 character matrix, which is a lot better! Furthermore, if replace the underscores with underlines,
then we can actually draw characters inside the cells. This allows for very nice looking animations.