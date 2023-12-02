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

TODO Lynn: From here!
  - Make a "picture" of all the compiler state for a small ResoCircuit
  - DUH! Inverse this. **Incidence map**
    - input_inc_wires
    - logic_inc_inputs
    - output_inc_inputs
    - output_inc_logics
    - wire_inc_outputs
    
    Each loop will look like this:

      temp_input_state = [False for _ in wires]
      for input_i in input_regions:
        for wire_i in input_inc_wires[input_i]:
          temp_input_state[input_i] ||= wire_state[wire_i]
      
      // proceed; inp reads from wire(start)
      // then each logic reads from inputs
      // then each output reads from inputs, from logics
      // then each wire(end) reads from output
    
    Note: Do it in five loops like the above:
      input -> wire
      logic -> input
      output -> logic
      output -> input
      end_wire -> output

    Reason: Doing it on one long nested loop (rather than five)
    means repeating work input -> wire

    end_wire -> output -> logic -> input -> wire
                       ----------> input -> wire

*/

// todo:
// this is super easy
// iterate over the dense indices, then their ortho neighbors

use crate::resel::{Resel};
use crate::reselboard::{ReselBoard};
use crate::RegionMap::{RegionMap};

struct AdjacencyMap {
  // Just a standard adjacency list, made denser
  // Uses dense-class indices
  wire_to_inputs:     Vec<Vec<usize>>,
  input_to_logics:    Vec<Vec<usize>>,
  input_to_outputs:   Vec<Vec<usize>>,
  output_to_wires:    Vec<Vec<usize>>
}

pub fn adjacencymap_from_regionmap(
  rm: &RegionMap
) {
  /*
  pub struct RegionMap {
    xy_to_region:     Vec<Vec<usize>>,
    region_to_xys:    Vec<Vec<(usize, usize)>>,
    region_to_resel:  Vec<Resel>,

    wire_regions:     Vec<usize>,
    input_regions:    Vec<usize>,
    logic_regions:    Vec<usize>,
    output_regions:   Vec<usize>,

    reverse_dense:    Vec<usize>
  }

  for each ri in rm.wire_regions,
    for each (x,y) in rm.region_to_xys:
      // iterate over neighbors
      for each nx, ny,
        n_ri = xy_to_region[x][y]

        if n_ri is input, 
          input_to_wires.push(
            reverse_index[n_ri]
          )
  */
}

// eof
// todo: tests