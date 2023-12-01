# ALGORITHMS.md

This is an overview of algorithms used in Reso.

## Connected-Component Labeling with per-class neighborhood convolutions

This algorithm is used when pre-processing an image to be compiled to a Reso Circuit. When I first programmed Reso in Python, I didn't know the name of the algorithm. (In fact, I only learned its name when I described it to ChatGPT.)

The typical preprocessing pipeline is like this:

1. Load an image and convert it to a Vec<Vec<Resel>> 
2. Identify contiguous regions of Resels. (We are here)
3. Compile the Reso circuit graph from the adjacent regions.

There are a few particulars to our implementation:

1. "On" and "off" wires belong to the same region. We use `resel.same(other_resel)` instead of `resel == other_resel`.
  - E.g. `Resel::WireOrangeOff != Resel::WireOrangeOn`, but `Resel::WireOrangeOff.same(Resel::WireOrangeOn)`
2. Wire regions are 8-connected (orthogonally + diagonally), but all other regions are 4-connected (orthogonally).
3. We also want to maintain lists of region indices per class.
  - E.g. With 5 regions, we might have something like `wires = [1, 3]`, `ands = [2,]`, `inputs = [0,]`, `outputs = [4,]`.
4. The algorithm maintains a `visited: Vec<Vec<bool>>` to keep track of which pixels were and were not visited.
5. The algorithm outputs a `region_map: Vec<Vec<usize>>`, where `0` represents `Resel::Empty`. So, region indices start at 1.

Here is the pseudocode for the region mapping algorithm. This might not be kept up to date; refer to `reselboard.rs`.

```
width, height = board.width, board.height
visited = (width, height) * False
region_idx = 0
region_to_xy = [[]]
xy_to_region = (width, height) * 0


for (x,y) in (width, height):
  if not visited[x][y]:
  
    if board[x][y] is empty:
      visited[x][y] = True
      region_to_xy[0].push((x,y))

    else:
      region_idx += 1
      color = board[x][y]
      region_to_xy.push([])

      neighbors = new queue()
      neighbors.push((x,y))

      while neighbors is not empty:
        x, y = neighbors.pop

        if board[x][y] == color:
          visited[x][y] = True
          xy_to_region[x][y] = region_idx
          region_to_xy[region_idx].push((x,y))

          for each (x,y) neighbor:
            if not visited[x][y]:
              neighbors.push((x,y))



    
```