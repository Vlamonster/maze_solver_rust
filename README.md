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
# Generate maze with 16 rows, 48 columns and the default delay of 25ms.
cargo run --release -- 16 48

# Generate maze with 16 rows, 48 columns and a delay of 0ms (instant).
cargo run --release -- 16 48 -d 0
```
I tested that this works on at least Windows 10, Ubuntu and macOS.
<details><summary>Example</summary>

![](example.gif)
</details>

# Generators
The following generators are included:
* Randomized depth-first search.
* Randomized breadth-first search.
* Kruskal's algorithm.

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
This is 5x5 character matrix to represent a 2x2 maze. We can do better by combining the vertical and horizontal walls into the same line:
```
_ ___   [H, N, H, H, H]
| |_|   [V, N, H, V, H]
|__ |   [V, H, N, N, V]

H: Horizontal,
V: Vertical,
N: None,
```
This gives us a 3x3 character matrix, which is a lot better!