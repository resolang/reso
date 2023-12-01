ARCHITECTURE and RESO CONCEPTS
===

TODO: This needs to be redone.

- Pixels/characters --> Resels
- Resels --> Regions, with Resel classes and indices
- ResoCircuit, with a graph of Resel nodes (regions) 

---

Reso is a toy for simulating logic circuits defined by pixel art. It does so by compiling regions of pixels into their corresponding logical elements, and compiling a logic graph from adjacent regions of pixels. The major inspiration is Minecraft's redstone and esolangs like Piet.

# Summary

The logic graph is an undirected graph of nodes of the following elements:

1. Wires carry a boolean state and are one of three colors.
  - Like insulation, the color of the wire do not matter.
2. Input nodes collect the state of all adjacent wire nodes into a boolean vector, i.e. $\{0,1\}^*$.
3. The two logic nodes (`AND` and `XOR`) calculate a single boolean based on the booleans of all adjacent input nodes.
4. The output nodes calculate the logical OR of all the bits stored in all adjacent input and output nodes.
5. At the end of each simulation loop, the wires are updated with the values from adjacent output nodes.

In this implementation, the logic graph of a Reso circuit is compiled from 2D  bitmap images. There is no reason a Reso circuit could not be defined by, say, a 3D bitmap, or a bitmap of ASCII characters, or an SVG, etc.

Here is how that process works:

1. The input is a bitmap image of RGB pixels with a given `(width, height)`
2. The bitmap image is converted pixel-per-pixel to a `ReselBoard: Vec<Vec<Resel>>`, where a `Resel` is an enum of one of the eleven Resel classes. (Six classes for wires, AND, XOR, input, output, and "empty".)
  - Six classes for wires `WireOrangeOff`, `WireOrangeOn`, `WireSapphireOff`, `WireSapphireOn`, `WireLimeOff`, `WireLimeOn`.
  - Four classes for `AND`, `XOR`, `Input`, `Output`.
  - One class for `Empty`, to which all other colors map.

3. Contiguous regions are calculated and given a region index `i`.
  - Wires are handled in a special way: Wires can be contiguous diagonally as well as orthogonally, and adjacent wire resels of the same color but different state (say, `WireLimeOff` and `WireLimeOn`) are considered the same region. (See `is_resel_same_class`
  - This data is stored in two mappings, `region_by_resel[x][y]->i` and  `resels_by_region[i]->[(x,y), ...]`.
  - Each region represents a node.

4. Adjacencies between regions are calculated to form the logic graph.
  - Only pertinent adjacencies are recorded: Input to logic, input to output, logic to output, and logic to wire.
5. This data is used by the Reso Circuit when simulating the circuit. 

# More details (WIP)

Under the hood, Reso works by taking a bitmap, mapping pixels to "resels" based on RGB values, finding contiguous regions of resels, and ultimately compiling that to a graph of logic nodes represented with adjacency lists. (This is performant assuming a sparse graph.) It's tempting to think of the wires as edges between logical elements, but wires are reso nodes too.

This program assumes you're using a 2D bitmap (i.e. an image), but this can be extended to anything! A 3D bitmap (like Minecraft), a 2D vector image (SVGs), etc.

---


Here's how creating a new Reso circuit works:

1. An **image** is loaded.
2. The `(w,h)` **image** is converted to a `(w,h)` **Resel Board** by mapping each **pixel** to a **Resel**.
  - Each **Resel** exists at an `(x,y)` coordinate and has a value.
  - There are `2^24` valid RGB pixels, but only `11` valid Resel **classes**:
    - **Wires**: orange, sapphire, lime, with boolean states.
    - **Input** nodes, which collect wire states.
    - **Logic** nodes XOR and AND, which compute their value from all adjacent input nodes.
    - **Output** nodes, which compute an OR from all adjacent input and logic node values.
    - (Input and output nodes are necessary to control the logic flow, since Reso compuiled to an undirected graph.)
    - The 11th Resel is simply the `Empty` (or "null" Resel). The majority of the sRGB color space maps to this, and can be thought of as a "comment".
3. From the **Resel Board**, we map to **regions** (short for "reso regions"). These are the smallest discrete element in our logic graph.
  - A **region** has an **integer index**, starting at 1.
    - Index '0' is for every Resel not a region, i.e. `Empty`.
  - Wire adjacency rules: Wires of the same color, on or off, are adjacent by orthogonal and diagonal neighbors.
  - Every other node (and, xor, input, output) are adjacent by orthogonal neighbors only.
  - The **mapping** is between Resel `x,y` and region integer index `i`.
    - **resels_by_region**: Map region `i` to list of resel `x,y`
      - E.g. `resels_by_region[3] = [(1,2),(1,3),...]`
    - **region_by_resels**: Map resel `x,y` to the region `i` it belongs to.
      - E.g. `region_by_resels[1][3] = 3`
4. These **regions of resels** form logical **nodes**. We record adjacency between regions to form the logic graph.
5. The end-result is a compiled **Reso circuit** which is ready for execution.
  - In this implementation, regions and nodes are one and the same. An optimization option to discard region information can be implemented here.


# Glossary and palette (WIP)

TLDR:

| Term  | Definition | 
| ----- | ---------- |
| **Images** | |
| Pixel | A three-byte RGB value at a given `x,y` position in a bitmap.
| Bitmap | A grid of pixels with a given `width, height`. Reso programs can be defined by bitmaps.
| **Resels** | |
| Resel | A Reso pixel with a class, has a given `x,y` position in a Resel board.
| Class | One of 11 values defining the functionality of a region.
| Resel board | A grid of resels with a given `width, height`. Reso programs can be defined by such a board.
| **Circuits** | |
| Region | A contiguous collection of Resel's with the same class, forming a node.
| Node  | The smallest logical unit in a Reso program. 
| Reso Circuit | A graph of nodes ready for simulation.

