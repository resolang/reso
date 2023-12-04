ARCHITECTURE and RESO CONCEPTS
===

You don't need to know any of this unless you are working on the code! If you are, you might benefit from the below.

```
TODO: Mermaid graph of dependencies.
```


# Core ideas

| Thing | Analogous thing | Where?                 | Thing, explained |
| ----- | --------------- | ---------------------- | ---------------- |
| Resel | Pixel           | `resel.rs` | Class that a circuit region can take on. (See the palette!) |
| ReselBoard | Image      | `reselboard.rs` | Just a grid `Vec<Vec<Resel>>` + supporting code. |
| RegionMap  | Select-by-color; nodes in a graph | `regionmap.rs` | Identifies the regions (nodes) in a Resel circuit. |
| Node / Region | Node in a graph | | Contiguous regions of resels form the logic circuit elements.
| IncidenceMap | Edges in a graph | `incidencemap.rs` | Circuits are graphs, and an incidence map is like an adjacency map, but it fits this use case better. |
| ResoCircuit | A logic graph | `resocircuit.rs` | The executable logic graph! (todo)

Reso is a toy for simulating logic circuits defined by pixel art. It does so by compiling regions of pixels into their corresponding logical elements, and compiling a logic graph from adjacent regions of pixels. The major inspiration is Minecraft's redstone and esolangs like Piet.

# Simulation algorithm summary

The logic graph is an undirected graph of nodes of the following elements.

In each iteration, 
1. **Wires carry bools:** Wires carry a boolean state, visually represented on the graph between iterations.
2. **Input collect wire states:** Input nodes collect the state of all adjacent wire nodes into a boolean vector, i.e. $\{0,1\}^*$.
3. **Logic nodes hold one bool:** The two logic nodes (`AND` and `XOR`) calculate a single boolean based on the booleans of all adjacent input nodes.
4. **Outputs OR over adjacent input, logic nodes:** The output nodes calculate the logical OR of all the bits stored in all adjacent input and logic nodes. An input node connected directly to an output node is equivalent to an "OR" gate.
5. **Wires updated at the end of each loop:** At the end of each simulation loop, the wires are updated with the values from adjacent output nodes. The state of the input, logic, and output nodes are temporary and are only used when calculating the iteration.

In this implementation, the logic graph of a Reso circuit is compiled from 2D  bitmap images. There is no reason a Reso circuit could not be defined by, say, a 3D bitmap, or a bitmap of ASCII characters, or an SVG, etc. 

# Compilation algorithm summary

1. A bitmap image is created as input
2. The bitmap image is converted pixel-per-pixel to a `ReselBoard: Vec<Vec<Resel>>`, where a `Resel` is an enum of one of the eleven Resel classes. (Six classes for wires, AND, XOR, input, output, and "empty".)
  - Six classes for wires `WireOrangeOff`, `WireOrangeOn`, `WireSapphireOff`, `WireSapphireOn`, `WireLimeOff`, `WireLimeOn`.
  - Four classes for `AND`, `XOR`, `Input`, `Output`.
  - One class for `Empty`, to which all other colors map.
3. Contiguous regions are calculated and given a region index `i`.
  - Wires are handled in a special way: Wires can be contiguous diagonally as well as orthogonally, and adjacent wire resels of the same color but different state (say, `WireLimeOff` and `WireLimeOn`) are considered the same region. (See `is_resel_same_class`
  - This data is stored in two mappings, `region_by_resel[x][y]->i` and  `resels_by_region[i]->[(x,y), ...]`.
  - Each region represents a node. See `regionmap.rs` for more info.
4. Adjacency between regions are calculated to form the logic graph.
5. This data is used by the Reso Circuit when simulating the circuit. 
