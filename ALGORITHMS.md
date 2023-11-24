# ALGORITHMS.md

This is an overview of algorithms used in Reso.

## Connected-Component Labeling with per-class neighborhood convolutions

This algorithm is used when pre-processing an image to be compiled to a Reso Circuit. When I first programmed Reso in Python, I didn't know the name of the algorithm. (In fact, I only learned its name when I described it to ChatGPT.)

The typical preprocessing pipeline is like this:

1. Load an image and convert it to a Vec<Vec<Resel>> 
2. Identify contiguous regions of Resels. (We are here)
3. Compile the Reso circuit graph from the adjacent regions.

There are two unique things with our implementation:

1. "On" and "off" wires belong to the same region. We use `resel.same(other_resel)` instead of `resel == other_resel`.
  - E.g. `Resel::WireOrangeOff != Resel::WireOrangeOn`, but `Resel::WireOrangeOff.same(Resel::WireOrangeOn)`
2. Wire regions are 8-connected (orthogonally + diagonally), but all other regions are 4-connected (orthogonally).
3. We also want to maintain lists of region indices per class.
  - E.g. With 5 regions, we might have something like `wires = [1, 3]`, `ands = [2,]`, `inputs = [0,]`, `outputs = [4,]`.

Here is the pseudocode for the region mapping algorithm:

```
for (x,y) in (width, height):
  
```