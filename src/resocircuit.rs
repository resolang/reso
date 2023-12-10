//! resocircuit.rs: Executable Reso circuits.

use crate::resel::{Resel};
use crate::reselboard::{
  ReselBoard,
  load_image_from_filename,
  image_to_reselboard,
};
use crate::regionmap::{RegionMap};
use crate::incidencemap::{IncidenceMap};

/*
todos:

- maintains to iterate:

  - input_state:  vec<vec<bool>> // shape: im.input_inc_wires
  - logic_state:  vec<bool>   // shape: rb.logic_regions
  - output_state: vec<bool>   // shape: rb.output_regions
  - wire_state:   vec<bool>   // shape: rb.wire_regions

  except for wire_state, these are only used during .iterate(),
  but not worth allocating/deallocating each run (... right?)

- impl From for...
  - ReselBoard
  - Image, vec<vec<char>>
  - Filename (png or txt)

- iterate():

  | for each | collect from incident | to             |
  | -------- | --------------------- | -------------- |
  | input    | wires                 | input_state    |
  | logic    | inputs                | logic_state    |
  | output   | inputs, logics        | output_state   |
  | wire     | output                | wire_state     |

  for each input: collect from incident wires
  for each logic: collect from incidence inputs
  for each output: collect from incident inputs, logics
  for each wire: collect from incident outputs

  man why did i put it in a table?

  also calls "update_image" by default

- update_image(): for each wire, update the state

- what else?
  - serialize/deserialize?
  - spew wire state over a port?
  - read wire state from a port?
*/

#[derive(Debug, Clone)]
pub struct ResoCircuit {
  pub rb: ReselBoard,
  pub rm: RegionMap,
  pub im: IncidenceMap,

  pub wire_state:   Vec<bool>,

  // Internal state used during .iterate()
  // (pre-allocated!)
  input_state:  Vec<Vec<bool>>,
  logic_state:  Vec<bool>,
  output_state: Vec<bool>,
}

impl From<ReselBoard> for ResoCircuit {
  fn from (rb: ReselBoard) -> ResoCircuit {
    let rm = RegionMap::from(&rb);
    let im = IncidenceMap::from(&rm);

    /* get state of all the wires

    python:
    wire_state = [
      rm.region_to_resel[region].wire_state()
      for region in rm.wire_regions
    ]

    regionmap currently takes wire region state as the
    first pixel (on or off) it encounters.

    this means something like `oOOOO` would be recorded
    as WireOrangeOff, because `o` would be encountered first.

    it _should_ update the entire region state on the subsequent
    `O` but we don't.

    this won't be a problem for nicely-defined circuits
    */
    let wire_state: Vec<bool> = rm.wire_regions.iter().map(
      |region| {
        rm.region_to_resel[*region]
        .wire_state().unwrap() // should always be bool
      }
    ).collect();

    /* prepare to allocate state for all the inputs
    input_state = Vec<Vec<bool>> of false
      in same shape as im.input_inc_wires
    
    input_state = [[False for _ in inc_wires]
      for inc_wires in im.input_inc_wires]
    */
    let mut input_state: Vec<Vec<bool>> = im.input_inc_wires
      .iter().map(
        |inc_wires| { vec![false; inc_wires.len()] }
      ).collect();
    
    let mut logic_state  = vec![false; rm.logic_regions.len()];
    let mut output_state = vec![false; rm.output_regions.len()];

    ResoCircuit{
      rb: rb,
      rm: rm,
      im: im,

      wire_state: wire_state,
      input_state: input_state,
      logic_state: logic_state,
      output_state: output_state,
    }
  }
}

// todo: impl from image, str vec, file?

impl ResoCircuit{

  /// Convenience function: Reset input_state, logic_state, output_state between iterations
  fn reset_intermediate_state(&mut self) {
    for ii in 0..self.input_state.len() {
      for wi in 0..self.input_state[ii].len() {
        self.input_state[ii][wi] = false
      }
    }
    for li in 0..self.logic_state.len() {
      self.logic_state[li] = false
    }

    for oi in 0..self.output_state.len() {
      self.output_state[oi] = false
    }
  }

  // TODO Lynn, from here!
  // - Break `iterate` into smaller &mut self methods
  // - TDD methods to fix them
  // - 
  pub fn iterate(&mut self) {

    // Collect input state vector from incident wires

    for (ii, inc_wires) in self.im.input_inc_wires.iter().enumerate() {
      // ii = input_index, inc_wires = list of wire_index
      for wi in inc_wires.iter() {
        self.input_state[ii][*wi] = self.wire_state[*wi]
      }
    }

    /* todo remove
    println!(
      "logic_regions={:?}\nreverse_dense={:?}\nregion_to_resel={:?}",
      self.rm.logic_regions,
      self.rm.reverse_dense,
      self.rm.region_to_resel,
    );
    */
    // Collect logic state from incident inputs
    for (li, inc_inputs) in self.im.logic_inc_inputs.iter().enumerate() {
      // li = logic_index, inc_inputs = list of input_index
      let lri = self.rm.logic_regions[li];//self.rm.reverse_dense[li];

      // For each incident input,
      for ii in inc_inputs.iter() {
        // ii = input_index
        
        if self.rm.region_to_resel[lri] == Resel::AND {
          self.logic_state[li] = (
            self.logic_state[li] || self.input_state[*ii].iter().fold(
              true, |acc, &x| acc && x // AND over inputs incident wires
            )
          );
        } else if self.rm.region_to_resel[lri] == Resel::XOR {
          self.logic_state[li] = (
            self.logic_state[li] || self.input_state[*ii].iter().fold(
              false, |acc, &x| acc ^ x // XOR over inputs incident wires
            )
          );
        } else {
          panic!(
            "rc.rm.region_to_resel[lri={}]={:?} is not logic?!?",
            lri, self.rm.region_to_resel[lri]
          );
        }
      }
    }


    // Collect output state from incident inputs
    for (oi, inc_inputs) in self.im.output_inc_inputs.iter().enumerate() {
      for ii in inc_inputs.iter() {
        self.output_state[oi] = (
          self.output_state[oi] || self.input_state[*ii].iter().fold(
            false, |acc, &x| acc || x // OR over input incident wires
          )
        )
      }
    }

    // Collect output state (continued) from incident logics
    for (oi, inc_logics) in self.im.output_inc_logics.iter().enumerate() {
      for li in inc_logics.iter() {
        self.output_state[oi] = self.output_state[oi] || self.logic_state[*li]
      }
    }

    // Collect wire state from incident logics
    for (wi, inc_outputs) in self.im.wire_inc_outputs.iter().enumerate() {
      self.wire_state[wi] = false;
      for oi in inc_outputs.iter() {
        self.wire_state[wi] = self.wire_state[wi] || self.output_state[*oi]
      }
    }

    // Cleanup
    self.reset_intermediate_state()
  }


  // fn to update image pixels

  // fn to get image

}

#[cfg(test)]
mod resocircuit_tests {
  use super::*;

  #[test]
  fn test_initialize_halfadder() {
    let mut rc = ResoCircuit::from(
      image_to_reselboard(
        load_image_from_filename(
          "./src/testing/test_half_adder_01.png"
        ).unwrap()
      )
    );

    // Check wire state
    for (ri, state) in [
      (0, true), (1, true), (2, true), (3, true)
    ] {
      assert_eq!(rc.wire_state[ri], state)
    }
    println!("{:?}", rc);

    rc.iterate();
    println!("\n\n\n{:?}", rc);
    for (ri, state) in [
      (0, true), (1, false), (2, false), (3, true)
    ] {
      assert_eq!(rc.wire_state[ri], state)
    }

    rc.iterate();
    println!("\n\n\n{:?}", rc);
    for (ri, state) in [
      (0, true), (1, false), (2, true), (3, false)
    ] {
      assert_eq!(rc.wire_state[ri], state)
    }
  }

}

// TODO Lynn, from here

// eof