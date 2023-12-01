

use std::collections::HashMap;
use image::{
  GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage
};

mod resel;
use resel::{Resel
};

mod reselboard;
use reselboard::{
  image_to_vecvecresel,
  load_image_from_filename,
  region_map_from_reselboard
};

//fn example_main_debug_a_board() {
fn main() {
  let board = image_to_vecvecresel(
    &load_image_from_filename(
      "./src/testing/test_01_new-palette.png"
    ).unwrap()
  );

  let (
    xy_to_region,
    region_to_xys,
    region_to_resel,
    wire_regions,
    input_regions,
    logic_regions,
    output_regions
  ) = region_map_from_reselboard(&board);

  println!("xy_to_region = {:?}", xy_to_region);
  println!("region_to_xys = {:?}", region_to_xys);
  println!("region_to_resel = {:?}", region_to_resel);
  println!("wire_regions = {:?}", wire_regions);
  println!("input_regions = {:?}", input_regions);
  println!("logic_regions = {:?}", logic_regions);
  println!("output_regions = {:?}", output_regions);

}

/*


/// Given a reselboard and the mapping between regions and resels,
/// get the resel class at each region, plus dense per-class indices.
/// (The dense class indices are used at runtime.)
/// Outputs: class_by_region, wire_nodes, input_nodes, output_nodes, logic_nodes
fn class_indices_from_reselboard_and_regions(
  reselboard: &Vec<Vec<Resel>>,
  region_by_resel: &Vec<Vec<usize>>,
  resels_by_region: &Vec<Vec<(usize, usize)>>,
) -> (Vec<Resel>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
  let mut class_by_region = vec![Resel::Empty; resels_by_region.len()];
  let mut wire_nodes = Vec::new();
  let mut input_nodes = Vec::new();
  let mut output_nodes = Vec::new();
  let mut logic_nodes = Vec::new();

  for (region_idx, resels) in resels_by_region.iter().enumerate() {
    // get resel_class from the first pixel in the region,
    // setting Resel::Empty if the region is empty
    let resel_class = match resels.len() {
      0 => Resel::Empty,
      _ => reselboard[resels[0].0][resels[0].1]
    };
    
    // Update our values
    class_by_region[region_idx] = resel_class;
    match resel_class {
      Resel::WireOrangeOff | Resel::WireOrangeOn |
      Resel::WireSapphireOff | Resel::WireSapphireOn |
      Resel::WireLimeOff | Resel::WireLimeOn => {
        wire_nodes.push(region_idx);
      },
      Resel::Input => {
        input_nodes.push(region_idx);
      },
      Resel::Output => {
        output_nodes.push(region_idx);
      },
      Resel::AND | Resel::XOR => {
        logic_nodes.push(region_idx);
      },
      _ => {}
    }
  }

  /*
  e.g.
  class_by_region: [
    Empty, WireOrangeOn, WireLimeOn, WireSapphireOff, Input, Output, AND, Input, Empty
  ]
  wire_nodes:      [1, 2, 3,]
  input_nodes:     [4, 7,]
  output_nodes:    [5,]
  logic_nodes:     [6,]
  */
  (class_by_region, wire_nodes, input_nodes, output_nodes, logic_nodes)
}

// todo lynn! current plan:
//  - Function to get adjacent region indices from a region
//  - Use that to populate input_to_wire, etc. five vars
fn get_adjacent_region_idxs(
  region_idx: usize,
  region_by_resel: &Vec<Vec<usize>>,
  resels_by_region: &Vec<Vec<(usize, usize)>>,
) -> Vec<usize> {
  let mut adjacent_regions = Vec::new();
  // warning: possibility of runtime error if called on non-grid VecVec
  let (width, height) = (region_by_resel.len(), region_by_resel[0].len());

  for (x, y) in resels_by_region[region_idx].iter() {
    // Adjacent regions are only adjacent by orthogonal
    for &(dx, dy) in vec![(1,0), (0,1), (width-1,0), (0,height-1)].iter() {
      let (neighbor_x, neighbor_y) = ((x + dx) % width, (y + dy) % height);


      let neighbor_region = region_by_resel[neighbor_x][neighbor_y];
      // check neighbor region is not the same, and not already in the list
      if neighbor_region != region_idx && !adjacent_regions.contains(&neighbor_region){
        adjacent_regions.push(neighbor_region);
      }
    }
  }
  adjacent_regions
}


#[derive(Debug, Clone)]
struct ResoCircuit {
  // aux drawing data
  image:            image::DynamicImage,
  reselboard:       Vec<Vec<Resel>>,
  region_by_resel:  Vec<Vec<usize>>,
  resels_by_region: Vec<Vec<(usize, usize)>>,

  // index regions by resel class
  // in addition to region index, this also maintains a dense index for wire, io, logic
  // (e.g. region 7 might be wire 3, so wire_nodes[3] == 7)
  class_by_region: Vec<Resel>, // length == number of total regions
  wire_nodes:      Vec<usize>, // length == number of wire regions
  input_nodes:     Vec<usize>,
  output_nodes:    Vec<usize>,
  logic_nodes:     Vec<usize>,

  // connectivity data between classes
  // uses the dense indices for wire_nodes, input_nodes, etc. above
  // (e.g. we might have input_nodes[4] == 8, and input_to_wire[4] == [3,]
  //  which means region 8 is input 4, and has incident wire 3, which is region 7)
  // (But you can ignore region index here, since we dense per-class indices.
  input_to_wire:   Vec<Vec<usize>>,  // input_idx -> wire_idx. (input nodes poll incident wires)
  // todo: Should the above be wire_to_input?
  input_to_logic:  Vec<Vec<usize>>,
  input_to_output: Vec<Vec<usize>>,
  logic_to_output: Vec<Vec<usize>>,
  output_to_wire:  Vec<Vec<usize>>,
  
  // temporary state data used at runtime
  wire_state:   Vec<bool>, // length == number of wire regions
  logic_state:  Vec<bool>,
  output_state: Vec<bool>,
}



fn main() {
  let filename = "test.png";
  let img = load_image_from_filename(filename);

  let (width, height) = img.dimensions();
  println!("Dimensions: {}x{}", width, height);
  println!("Reselboard: {:?}", image_to_reselboard(&img));
  let (region_by_resel, resels_by_region) = 
    resel_region_mapping_from_reselboard(&image_to_reselboard(&img));
  println!("Region by resel:\n{:?}", region_by_resel);
  println!("Resel by region:\n{:?}", resels_by_region);

  let (class_by_region, wire_nodes, input_nodes, output_nodes, logic_nodes) = 
    class_indices_from_reselboard_and_regions(
      &image_to_reselboard(&img),
      &region_by_resel,
      &resels_by_region,
    );
  /*
  println!("Class by region:\n{:?}", class_by_region);
  println!("Wire nodes:\n{:?}", wire_nodes);
  println!("Input nodes:\n{:?}", input_nodes);
  println!("Output nodes:\n{:?}", output_nodes);
  println!("Logic nodes:\n{:?}", logic_nodes);

  // for each region, print adjacent regions
  for (region_idx, region) in resels_by_region.iter().enumerate() {
    println!("Region {} has {} resels", region_idx, region.len());
    println!("  Adjacent regions: {:?}", get_adjacent_region_idxs(
      region_idx, &region_by_resel, &resels_by_region
    ));
    // also print enum of adjacent region idxs
    println!("  Adjacent regions: {:?}", get_adjacent_region_idxs(
      region_idx, &region_by_resel, &resels_by_region
    ).iter().map(|&idx| class_by_region[idx]).collect::<Vec<Resel>>());
  }
  */

}

*/