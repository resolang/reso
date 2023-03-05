# ARCHITECTURE and RESO CONCEPTS

Reso is a toy for drawing and executing circuits. It works by connecting digital logic elements. **Wires** carry bool values to **input nodes**, which carry values to **logic** and **output** nodes. The output nodes then connect to other **wires.** The major inspiration is Minecraft's redstone.

Under the hood, Reso works by compiling to a graph of logic elements, represented using adjacency lists. (We kind of assume the graph is sparse.) It's tempting to think of the wires as edges, but wires are reso nodes too.

This program assumes you're using a 2D bitmap (i.e. an image), but this can be extended to anything! A 3D bitmap (like Minecraft), a 2D vector image (SVGs), etc.
