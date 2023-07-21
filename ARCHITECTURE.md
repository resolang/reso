# ARCHITECTURE and RESO CONCEPTS

TODO: this ugley

Reso is a toy for simulating logic circuits defined by pixel art. **Wires** carry boolean values to **input nodes**, which carry values to **logic** and **output** nodes. The output nodes then connect to other **wires.** The major inspiration is Minecraft's redstone and esolangs like Piet.

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

TLDR:

| Term  | Definition |
| ----- | ---------- |
| Pixel | A three-byte RGB value at a given `x,y` position in a bitmap
| Bitmap | A grid of pixels with a given `width, height`. Reso programs can be defined by bitmaps.
| Resel | A Reso element with a class at a given `x,y` position in a Reso board.
| Resel board | A grid of resels with a given `width, height`. Reso programs can be defined by such a board.
| Class | One of 11 values defining the functionality of a region.
| Region | A contiguous collection of Resel's with the same class, forming a node.
| Node  | The smallest logical unit in a Reso program. 
| Reso Circuit | A graph of nodes ready for execution.

TLDR plus technical details:

| Term  | Definition |
| ----- | ---------- |
| Region | Has index `i`, see `resels_by_region[i] -> [(x,y),...]` and `region_by_resels[x][y] -> i`



A **Reso Circuit** contains:

 - (Optional) for printing:
  - The original input image. This is to preserve 'comments' when exporting, because they are not preserved when mapping to ReselBoard.
 - 