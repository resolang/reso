

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