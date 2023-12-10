//! incidencemap.rs: Functionality to identify the useful incidences between regions.
//!  
//! Used when executing the circuit.
//!  
//! Specifically,
//!   inputs  <- wires  
//!   logic   <- inputs 
//!   outputs <- inputs 
//!   outputs <- logic  
//!   wires   <- output
//! 
//! This makes use of the dense indices provided by Regionmapper.
//! 
//! E.g. 
//!   Reselboard:   xy_to_region:
//!   ..........    ..........
//!  :    =lll     :    4888
//!  :ooo+^        :11135
//!  :sss+&        :22236
//!  :    =lll     :    7999
//!   ..........    ..........
//! 
//! 
//! - wire_regions   = [1,2,8,9]
//! - input_regions  = [3,]
//! - logic_regions  = [5,6,]
//! - output_regions = [4,7,]
//! - reverse_dense  = [0,0,1,0,0,0,1,1,2,3]
//! 
//! We will get:
//! - input_inc_wires   = [[0,1],]   // because adj(3,1) and adj(3,2)
//! - logic_inc_inputs  = [[0],[0],] // because adj(5,3) and adj(6,3)
//! - output_inc_inputs = [[], []]   // because no adj(4,x), no adj(7,x)
//! - output_inc_logics = [[0], [1]] // because adj(4,5) and adj(7,6)
//! - wire_inc_outputs  = [[], [], [0], [1]] // because adj(8,4) and adj(9,7)
//! 
//! 
//! These incidence maps are based on the dense index of each list of regions.
//! 
//! e.g.                          // region 9 adj region 7
//! - wire_regions[3]     == 9    // wire_index    3 == region_index 9
//! - wire_inc_outputs[3] == [1,] // output_region 1 == region_index 7
//! - output_regions[1]   == 7
//! 
//! TODO: From here!
//! - More tests
//! - Review / clean this up
//! - Then, CLI in main
//! - Then, ARCHITECTURE.md, ALGORITHMS.md, README.md
//! - Then, ResoCircuit

use crate::resel::{Resel};
use crate::regionmap::{RegionMap};

/// IncidenceMap holds mapping of incident dense-region indices,
/// based on a RegionMap.
/// 
/// For example, given `input_inc_wires[2] = [1,2]` means
/// RegionMap.input_nodes[2] is adjacent to both RegionMap.wire_nodes[1] and
/// RegionMap.wire_nodes[2].
/// 
/// This is used when executing a circuit.
pub struct IncidenceMap {
  pub input_inc_wires:    Vec<Vec<usize>>,
  pub logic_inc_inputs:   Vec<Vec<usize>>,
  pub output_inc_inputs:  Vec<Vec<usize>>,
  pub output_inc_logics:  Vec<Vec<usize>>,
  pub wire_inc_outputs:   Vec<Vec<usize>>,
}

/// Returns an IncidenceMap from a RegionMap
fn incidencemap_from_regionmap(
  rm: &RegionMap
) -> IncidenceMap {
  // todo redundant: There's a lot of repeating here
  // Surely there is some way to repeat less 
  let mut input_inc_wires:    Vec<Vec<usize>> = vec![];
  let mut logic_inc_inputs:   Vec<Vec<usize>> = vec![];
  let mut output_inc_inputs:  Vec<Vec<usize>> = vec![];
  let mut output_inc_logics:  Vec<Vec<usize>> = vec![];
  let mut wire_inc_outputs:   Vec<Vec<usize>> = vec![];

  // e.g. input_inc_wires[dense_input_index] -> list of dense wire indices

  
  // Can't iterate over closures unless you "Box" them
  // I do not understand this and I don't like it
  // Look how ugly this is
  for (x_inc_y, x_regions, y_condition) in [
    (&mut input_inc_wires,   &rm.input_regions, //|y| y.is_wire()),
      Box::new(|y: Resel| y.is_wire()) as Box<dyn Fn(Resel) -> bool>),
    (&mut logic_inc_inputs,  &rm.logic_regions, //|y| y.is_input()),
      Box::new(|y: Resel| y.is_input()) as Box<dyn Fn(Resel) -> bool>),
    (&mut output_inc_inputs, &rm.output_regions,//|y| y.is_input()),
      Box::new(|y: Resel| y.is_input()) as Box<dyn Fn(Resel) -> bool>),
    (&mut output_inc_logics, &rm.output_regions,//|y| y.is_logic()),
      Box::new(|y: Resel| y.is_logic()) as Box<dyn Fn(Resel) -> bool>),
    (&mut wire_inc_outputs,  &rm.wire_regions,  //|y| y.is_output()),
      Box::new(|y: Resel| y.is_output()) as Box<dyn Fn(Resel) -> bool>),
  ] {
    for (x_i, ri) in x_regions.iter().enumerate() {
      assert_eq!(x_i, x_inc_y.len());
      x_inc_y.push(vec![]);

      for adj_ri in rm.get_adjacent_regions(*ri) {

        if y_condition(rm.region_to_resel[adj_ri]) {
          x_inc_y[x_i].push(rm.reverse_dense[adj_ri])
        }
      }
    }
  }
  

  IncidenceMap{
    input_inc_wires,
    logic_inc_inputs,
    output_inc_inputs,
    output_inc_logics,
    wire_inc_outputs,
  }
}

impl From<RegionMap> for IncidenceMap {
  fn from(rm: RegionMap) -> IncidenceMap {
    incidencemap_from_regionmap(&rm)
  }
}

impl From<&RegionMap> for IncidenceMap {
  fn from(rm: &RegionMap) -> IncidenceMap {
    incidencemap_from_regionmap(rm)
  }
}

// todo: impl from RB, image, vec str, file?


#[cfg(test)]
mod reselboard_tests {

  use super::*;
  use crate::reselboard::{
    load_image_from_filename,
    image_to_reselboard,
  };
  use crate::regionmap::{RegionMap};

  #[test]
  fn test_incident_map_on_half_adder() {
    let rb = image_to_reselboard(
      load_image_from_filename(
        "./src/testing/test_half_adder.png"
      ).unwrap()
    );

    let rm = RegionMap::from(&rb);
    let im = IncidenceMap::from(&rm);//incidencemap_from_regionmap(&rm);

    assert_eq!(
      im.input_inc_wires,
      vec![vec![0,1]]
    );
    assert_eq!(
      im.logic_inc_inputs,
      vec![vec![0], vec![0]]
    );
    assert_eq!(
      im.output_inc_inputs,
      vec![vec![], vec![]]
    );
    assert_eq!(
      im.output_inc_logics,
      vec![vec![0], vec![1],]
    );
    assert_eq!(
      im.wire_inc_outputs,
      vec![vec![],vec![],vec![0],vec![1],]
    );
  }
}

// eof