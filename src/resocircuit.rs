//! resocircuit.rs: Execut
//! 
//! 

/*
todos:

- maintains to iterate:

  - input_state:  vec<vec<bool>> // shape: im.input_inc_wires
  - logic_state:  vec<bool>   // shape: rb.logic_regions
  - output_state: vec<bool>   // shape: rb.output_regions
  - wire_state:   vec<bool>   // shape: rb.wire_regions

  except for wire_state, these are only used during .iterate(),
  but not worth allocating/deallocating each run (... right?)

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

*/