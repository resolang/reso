/* adjacencymap.rs

Functionality to identify the useful adjacencies between regions.

Specifically,
 wires  -> inputs
 inputs -> logic
 inputs -> outputs
 logic  -> outputs
 outputs -> wires

This makes use of the dense indices provided by Regionmapper:

Reselboard:   xy_to_region:
  O+&=L         12456
   S             3


-  wire_regions = [1, 3, 6]
- input_regions = [2,]
- logic_regions = [4,]
- output_regions = [5,]

We will get:
- wire_to_inputs   = [[0,], [0,], []]
- input_to_logics  = [[0,]]
- input_to_outputs = []
- output_to_wires  = [[2,]]

These adjacency maps are based on the dense index of each list of regions.
  
E.g. `wire_regions[1] == 3`, i.e. wire index 1 is region index 3
`wire_to_inputs[1] == [0,]`, i.e. wire index 1 is adjacent to input index 0
`input_to_logics[0] == [0]`
`input_to_outputs 
*/