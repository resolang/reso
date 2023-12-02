/* incidencemap.rs

Functionality to identify the useful incidences between regions.

Specifically,
  inputs  <- wires  
  logic   <- inputs 
  outputs <- inputs 
  outputs <- logic  
  wires   <- output

This makes use of the dense indices provided by Regionmapper.

E.g. 
  Reselboard:   xy_to_region:
  ..........    ..........
 :    =lll     :    4888
 :ooo+^        :11135
 :sss+&        :22236
 :    =lll     :    7999
  ..........    ..........


- wire_regions   = [1,2,8,9]
- input_regions  = [3,]
- logic_regions  = [5,6,]
- output_regions = [4,7,]
- reverse_dense  = [0,0,1,0,0,0,1,1,2,3]

We will get:
- input_inc_wires   = [[0,1],]   // because adj(3,1) and adj(3,2)
- logic_inc_inputs  = [[0],[0],] // because adj(5,3) and adj(6,3)
- output_inc_inputs = [[], []]   // because no adj(4,x), no adj(7,x)
- output_inc_logics = [[0], [1]] // because adj(4,5) and adj(7,6)
- wire_inc_outputs  = [[], [], [0], [0]] // because adj(8,4) and adj(9,7)


These incidence maps are based on the dense index of each list of regions.

e.g.                          // region 9 adj region 7
- wire_regions[3]     == 9    // wire_index    3 == region_index 9
- wire_inc_outputs[3] == [1,] // output_region 1 == region_index 7
- output_regions[1]   == 7

TODO: From here!
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

    Reason: Doing it on one long nested loop (rather than five)
    means repeating work input -> wire
      end_wire -> output -> logic -> input -> wire
                        ----------> input -> wire

*/

use crate::resel::{Resel};
use crate::reselboard::{ReselBoard};
use crate::RegionMap::{RegionMap};

struct IncidenceMap {
  input_inc_wires:    Vec<Vec<usize>>,
  logic_inc_inputs:   Vec<Vec<usize>>,
  output_inc_inputs:  Vec<Vec<usize>>,
  output_inc_logics:  Vec<Vec<usize>>,
  wire_inc_outputs:   Vec<Vec<usize>>,
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
  */


  /*
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