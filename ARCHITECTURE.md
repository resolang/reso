# ARCHITECTURE and RESO CONCEPTS

Reso is a toy for drawing and executing circuits. It works by connecting digital logic elements. **Wires** carry bool values to **input nodes**, which carry values to **logic** and **output** nodes. The output nodes then connect to other **wires.** The major inspiration is Minecraft's redstone.

Under the hood, Reso works by compiling to a graph of logic elements, represented using adjacency lists. (We kind of assume the graph is sparse.) It's tempting to think of the wires as edges, but wires are reso nodes too.

This program assumes you're using a 2D bitmap (i.e. an image), but this can be extended to anything! A 3D bitmap (like Minecraft), a 2D vector image (SVGs), etc.

---


Here's how creating a new Reso circuit works:

1. An **image** is loaded.
2. The `(w,h)` **image** is converted to a `(w,h)` **Reso Board** by mapping each **pixel** to a **Resel**.
  - Each **Resel** exists at an `(x,y)` coordinate and has a value.
  - There are `2^24` valid RGB pixels, but only `11` valid Resels!
  - Six **wire** resels: There are three wire colors (orange, sapphire, lime) and two wire states, for `2*3=6` total colors.
  - The **input** resel connects wires to logic, and the **output** resel connects logic to wires. These are necessary because Reso does not compile to a directed graph.
  - The **XOR** and **AND** resels also help with logic. (**OR** is implicit  by connecting an input node directly to output.)
  - Finally, there is an `Empty` Resel, for any unmapped element.
  - Note: Any non-Resel pixels are left out. So 'comments' in a circuit are lost in the mapping.
3. From the **Reso Board**, we map to **Regions** (short for "reso regions"). These are the smallest discrete element in our logic graph.
  - A **region** has an **integer index**, starting at 1.
    - Index '0' is for every Resel not a region, i.e. `Empty`.
  - Wire adjacency rules: Wires of the same color, on or off, are adjacent by orthogonal and diagonal neighbors.
  - Every other node (and, xor, input, output) are adjacent by orthogonal neighbors only.
  - In the code, this is represented by two values:
    - **resels_by_region**: 

A **Reso Circuit** contains:

 - (Optional) for printing:
  - The original input image. This is to preserve 'comments' when exporting, because they are not preserved when mapping to ResoBoard.
 - 