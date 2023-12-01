
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
};
mod regionmap;
use regionmap::{
  region_map_from_reselboard,
  RegionMap
};


/*
todo:
- Reselboard.rs: See file
- regionmap.rs: reverse dense index, see file

- Then:
  - AdjacencyMap.rs
  - ResoCircuit.rs
  - main CLI
- Far later:
  - resolang.dev site
  - IDE
*/

/*
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