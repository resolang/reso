

// Codex generated
#[derive(Debug, Clone)]
struct ResoCircuit {
    // aux drawing data
    resels_by_region: Vec<Vec<(usize, usize)>>,
    region_by_resel: Vec<Vec<usize>>,

    // compiled graph
    class_by_region: Vec<Resel>,
    wire_nodes: Vec<usize>,
    input_nodes: Vec<usize>,
    logic_nodes: Vec<usize>,
    output_nodes: Vec<usize>,
    wires_to_inputs: HashMap<usize, Vec<usize>>,
    inputs_to_logic: HashMap<usize, Vec<usize>>,
    inputs_to_outputs: HashMap<usize, Vec<usize>>,
    outputs_to_wires: HashMap<usize, Vec<usize>>,

    // temp vars which change while running
    wire_state: Vec<bool>,
    input_state: Vec<bool>,
    output_state: Vec<bool>,
}

// copilot generated
impl ResoCircuit {
  fn new(original_image: image::DynamicImage) -> Self {
      let (width, height) = original_image.dimensions();
      let reselboard = image_to_reselboard(&original_image);
      let (region_by_resel, resels_by_region) = 
          resel_region_mapping_from_reselboard(&reselboard, width as usize, height as usize);
      let class_by_region = resels_by_region.iter().map(|resels| {
          reselboard[resels[0].0][resels[0].1]
      }).collect();
      let wire_nodes = class_by_region.iter().enumerate().filter_map(|(i, class)| {
          if [
              Resel::WireOrangeOff, Resel::WireOrangeOn,
              Resel::WireSapphireOff, Resel::WireSapphireOn,
              Resel::WireLimeOff, Resel::WireLimeOn
          ].contains(class) {
              Some(i)
          } else {
              None
          }
      }).collect();
      let input_nodes = class_by_region.iter().enumerate().filter_map(|(i, class)| {
          if [
              Resel::Input
          ].contains(class) {
              Some(i)
          } else {
              None
          }
      }).collect();
      let logic_nodes = class_by_region.iter().enumerate().filter_map(|(i, class)| {
          if [
              Resel::AND, Resel::XOR
          ].contains(class) {
              Some(i)
          } else {
              None
          }
      }).collect();
      let output_nodes = class_by_region.iter().enumerate().filter_map(|(i, class)| {
          if [
              Resel::Output
          ].contains(class) {
              Some(i)
          } else {
              None
          }
      }).collect();
      Self {
          original_image,
          reselboard,
          region_by_resel,
          resels_by_region,
          class_by_region,
          wire_nodes,
          input_nodes,
          logic_nodes,
          output_nodes,
      }
  }
}


// !!! Codex generated scrap below !!!


/// version of resel_region_mapping_from_reselboard
/// except width and height are inferred from reselboard
/// *COPILOT GENERATED* todo: Yeah, this slaps, just place this in?
fn resel_region_mapping_from_reselboard_2(
  reselboard: &Vec<Vec<Resel>>,
) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {
  let (width, height) = (reselboard.len(), reselboard[0].len());
  resel_region_mapping_from_reselboard(reselboard, width, height)
}

/* todo: struct ResoCircuit
  // aux drawing data
  - resels_by_region, region_by_resel

  // compiled graph
  - class_by_region
  - wire_nodes, input_nodes, logic_nodes, output_nodes
  - wires_to_inputs, inputs_to_logic, inputs_to_outputs, outputs_to_wires
      ... sparse adjacency via hashmap? (custom hashmap region idx -> vec<region idx>?) we expect sparse indexing ints to ints
      ... direct adjacency matrix? (num regions x num regions)?
      ... sparse adjacency via vec? vec<vec<usize>>, mostly empty. e.g. [[], [], [], [3, 4], []] etc
      ... Just use HashMap, probably, then revisit when I know more

  // temp vars which change while running
  - wire_state
  - input_state
  - output_state
*/


/*
/// Returns input_to_wire, input_to_logic, input_to_output, logic_to_output, output_to_wire
fn region_to_region_mappings_by_class(
  reselboard: &Vec<Vec<Resel>>,
  region_by_resel: &Vec<Vec<usize>>,
  resels_by_region: &Vec<Vec<(usize, usize)>>,
  class_by_region: &Vec<Resel>,
  wire_nodes: &Vec<usize>,
  input_nodes: &Vec<usize>,
  output_nodes: &Vec<usize>,
  logic_nodes: &Vec<usize>,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>) {
  let mut input_to_wire   
  let mut input_to_logic  = vec![Vec::new(); input_nodes.len()];
  let mut input_to_output = vec![Vec::new(); input_nodes.len()];
  let mut logic_to_output 
  let mut output_to_wire  

  let (width, height) = (reselboard.len(), reselboard[0].len());

  for (region_idx, resels) in resels_by_region.iter().enumerate() {
    let resel_class = class_by_region[region_idx];
    match resel_class {
      Resel::Input => {
        let input_idx = input_nodes.iter().position(|&x| x == region_idx).unwrap();
        for (x, y) in resels.iter() {
          for dx in -1..2 {
            for dy in -1..2 {
              if dx == 0 && dy == 0 { continue; }
              let (nx, ny) = ((x + dx + width) % width, (y + dy + height) % height);
              let neighbor_region = region_by_resel[nx][ny];
              let neighbor_class = class_by_region[neighbor_region];
              match neighbor_class {
                Resel::WireOrangeOff | Resel::WireOrangeOn |
                Resel::WireSapphireOff | Resel::WireSapphireOn |
                Resel::WireLimeOff | Resel::WireLimeOn => {
                  let wire_idx = wire_nodes.iter().position(|&x| x == neighbor_region).unwrap();
                  input_to_wire[input_idx].push(wire_idx);
                },
                Resel::AND | Resel::XOR => {
                  let logic_idx = logic_nodes.iter().position(|&x| x == neighbor_region).unwrap();
                  input_to_logic[input_idx].push(logic_idx);
                },
                Resel::Output => {
                  let output_idx = output_nodes.iter().position(|&x| x == neighbor_region).unwrap();
                  input_to_output[input_idx].push(output_idx);
                },
                _ => {}
              }
            }
          }
        }
      },
      Resel::AND | Resel::XOR => {
        let logic_idx = logic_nodes.iter().position(|&x| x == region_idx).unwrap();
        for (x, y) in resels.iter() {
          for dx in -1..2 {
            for dy in -1..2 {
              if dx == 0 && dy == 0 { continue; }
              let (nx, ny) = ((x + dx + width) % width, (y + dy + height) % height);
              let neighbor_region = region_by_resel[nx][ny];
              let neighbor_class = class_by_region[neighbor_region];
              match neighbor_class {
                Resel::Output => {
                  let output_idx = output_nodes.iter().position(|&x

  input_to_wire, input_to_logic, input_to_output, logic_to_output, output_to_wire

}
*/