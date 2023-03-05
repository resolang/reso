use std::collections::HashMap;
use image::{GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage};


/// Return image::DynamicImage given filename
fn load_image_from_filename(filename: &str) -> DynamicImage {
  // todo -- should be String and not &str?
  // Load the image from the file (copilot)
  let img = image::open(filename).expect("File not found, sorry!");
  let (width, height) = img.dimensions();
  println!("Loaded {} ({}x{} px).", &filename, width, height);
  img
}

/// Enum of all the classes a resel can have
#[derive(Debug, Clone, Copy, PartialEq)]
enum Resel {
  WireOrangeOff,
  WireOrangeOn,
  WireSapphireOff,
  WireSapphireOn,
  WireLimeOff,
  WireLimeOn,
  AND,
  XOR, 
  Input,
  Output,
  Empty
}

/// Mapping of of (R, G, B, A)  to Resel class.
fn rgbas_to_resel(r: u8, g: u8, b: u8, a: u8) -> Resel {
  match (r, g, b, a) {
    (128,  64,   0, 255) => Resel::WireOrangeOff,
    (255, 128,   0, 255) => Resel::WireOrangeOn,
    (  0,  64, 128, 255) => Resel::WireSapphireOff,
    (  0, 128, 255, 255) => Resel::WireSapphireOn,
    (64,  128,   0, 255) => Resel::WireLimeOff,
    (128, 255,   0, 255) => Resel::WireLimeOn,
    (  0, 128,  64, 255) => Resel::AND,
    (  0, 255, 128, 255) => Resel::XOR,
    ( 64,   0, 128, 255) => Resel::Input,
    (128,   0, 255, 255) => Resel::Output,
    _ => Resel::Empty,
  }
}

/// Mapping of image::Rgba<u8> to Resel class.
fn rgba_to_resel(pixel: Rgba<u8>) -> Resel {
  rgbas_to_resel(pixel[0], pixel[1], pixel[2], pixel[3])
}

/// Mapping of Resel class to RGBA value.
fn resel_to_rgba(resel: Resel) -> Rgba<u8> {
  match resel {
    Resel::WireOrangeOff   => Rgba([128,  64,   0, 255]),
    Resel::WireOrangeOn  => Rgba([255, 128,   0, 255]),
    Resel::WireSapphireOff => Rgba([  0,  64, 128, 255]),
    Resel::WireSapphireOn  => Rgba([  0, 128, 255, 255]),
    Resel::WireLimeOff   => Rgba([64,  128,   0, 255]),
    Resel::WireLimeOn    => Rgba([128, 255,   0, 255]),
    Resel::AND       => Rgba([  0, 128,  64, 255]),
    Resel::XOR       => Rgba([  0, 255, 128, 255]),
    Resel::Input       => Rgba([ 64,   0, 128, 255]),
    Resel::Output      => Rgba([128,   0, 255, 255]),
    Resel::Empty       => Rgba([0, 0, 0, 0])
  }
}

// Map an rgba image to a 2D grid of Resels.
fn image_to_reselboard(img: &DynamicImage) -> Vec<Vec<Resel>> {
  let (width, height) = img.dimensions();
  let mut reselboard = vec![vec![Resel::Empty; height as usize]; width as usize];
  for x in 0..width {
    for y in 0..height {
      let pixel = img.get_pixel(x, y);
      let resel = rgba_to_resel(pixel);
      reselboard[x as usize][y as usize] = resel;
    }
  }
  reselboard
  // TODO! This can be a fixed size array, I think?
  // see https://stackoverflow.com/questions/59164456/
}

/// Unusued; for text-based reselboard.
fn ascii_to_resel(c: char) -> Resel {
  match c {
    'o' => Resel::WireOrangeOff,
    'O' => Resel::WireOrangeOn,
    's' => Resel::WireSapphireOff,
    'S' => Resel::WireSapphireOn,
    'l' => Resel::WireLimeOff,
    'L' => Resel::WireLimeOn,
    '&' => Resel::AND,
    '^' => Resel::XOR,
    '+' => Resel::Input,
    '=' => Resel::Output,
    ' ' => Resel::Empty,
    _ => Resel::Empty,
  }
}

/// Unusued; for text-based reselboard.
fn resel_to_ascii(resel: Resel) -> char {
  match resel {
    Resel::WireOrangeOff   => 'o',
    Resel::WireOrangeOn  => 'O',
    Resel::WireSapphireOff => 's',
    Resel::WireSapphireOn  => 'S',
    Resel::WireLimeOff   => 'l',
    Resel::WireLimeOn    => 'L',
    Resel::AND       => '&',
    Resel::XOR       => '^',
    Resel::Input       => '+',
    Resel::Output      => '=',
    Resel::Empty       => ' ',
  }
}


// TODO: If an on and off wire are adjacent, set all as 'on'

/// Given a reselboard, find and index regions of adjacent elements.
/// Returns tuple (region_by_resel[x][y]->i, resels_by_region[i]->[(x,y), ...])
fn resel_region_mapping_from_reselboard(
  reselboard: &Vec<Vec<Resel>>,
  width: usize,
  height: usize,
) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {

  let mut region_idx: usize = 0;
  let mut visited:   Vec<Vec<bool>> = vec![vec![false; height as usize]; width as usize];
  // todo: visited is redundant, just check region_by_resel? defaults to 0
  let mut region_by_resel: Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
  let mut resels_by_region: Vec<Vec<(usize, usize)>> = vec![Vec::new()];

  // resels_by_region[0] empty-- we index regions starting with 1.
  // 'region_by_resel[x][y] = 0' means [x][y] doesn't have a region assignment
  resels_by_region.push(Vec::new());

  // todo: This can run class-by-region as well.
  // (... And all the elements in the compilation step!)
  // Wire nodes: Set as off by default, to "on" if we see one

  // For each resel
  for x in 0..width {
    for y in 0..height {
      if visited[x][y] {
        // Already visited; pass
      } else if reselboard[x][y] == Resel::Empty {
        // Mark as visited, but Empty does not make a Region. No work to do here.
        visited[x][y] = true
      } else {
        // Unvisited Resel marks a new region.
        // Update our region count, and prepare to mark new resels.
        // (region_idx 0 skipped intentionally; resels_by_region[0] stays empty.
        //  On first loop, region_idx == 1, resels_by_region.len() == 2)
        region_idx += 1;
        resels_by_region.push(Vec::new());

        // Explore neighbors one-by-one, starting with our first resel at (x,y)
        let mut neighbors: Vec<(usize, usize)> = Vec::new();        
        neighbors.push((x, y));

        // `neighbors` is only empty once all regions have been explored.
        while !neighbors.is_empty() {
          let (x, y) = neighbors.pop().unwrap();
          // (x,y) is a new resel in our region. Mark it
          region_by_resel[x][y] = region_idx;
          resels_by_region[region_idx].push((x,y));
          visited[x][y] = true;

          // Check contiguity to add neighbors in direction (dx, dy)
          for (dx, dy) in {
            // contiguity is orthogonal. wires can also be diagonally orthogonal
            if [
              Resel::WireOrangeOff, Resel::WireOrangeOn,
              Resel::WireSapphireOff, Resel::WireSapphireOn,
              Resel::WireLimeOff, Resel::WireLimeOn
            ].contains(&reselboard[x][y]) {
              // Diagonal and orthogonal neighbors
              [(1,0), (1, height-1), (0, height-1), (width-1, height-1),
               (width-1, 0), (width-1, 1), (0, 1), (1,1)]
            } else if [
              Resel::AND, Resel::XOR, Resel::Input, Resel::Output
            ].contains(&reselboard[x][y]) {
              // Orthogonal neighbors only. (dx,dy = 0,0 is skipped below.
              [(1,0), (0,height-1), (width-1, 0), (0, 1), (0,0), (0,0), (0,0), (0,0)]
            } else { // No neighbors, dead case.
              [(0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)]
            }
          }.iter() { // for (dx, dy) in ..neighbors to check.. {
            match (
              &reselboard[x][y], // Current resel
              &reselboard[(x + dx) % width][(y + dy)%height] // Neighbor resel
            ) {
              // Simple case: Resels match
              (resel_a, resel_b) if (
                // resel_a matches resel_b, not the originating pixel, and not already visited
                resel_a == resel_b && *dx != 0 && *dy != 0 && !visited[(x + dx) % width][(y + dy)%height]
              ) => {neighbors.push(((x + dx) % width, (y + dy)%height));},
              // Wires case: Match, but have different on/off values, add the resel to our bag of neighbors
              (
                // e.g. current resel and adjacent resel are orange
                Resel::WireOrangeOff | Resel::WireOrangeOn,
                Resel::WireOrangeOn  | Resel::WireOrangeOff
              ) | (
                // e.g. current resel and adjacent resel are sapphire
                Resel::WireSapphireOff | Resel::WireSapphireOn,
                Resel::WireSapphireOn  | Resel::WireSapphireOff
              ) | (
                // e.g. current resel and adjacent resel are lime
                Resel::WireLimeOff | Resel::WireLimeOn,
                Resel::WireLimeOn | Resel::WireLimeOff
              // ..., not the originating pixel, and not already visited
              ) if (*dx != 0 && *dy != 0 && !visited[(x + dx) % width][(y + dy)%height]) => {
                neighbors.push(((x + dx) % width, (y+ dy)%height))
              },
              // Else, do nothing
              (_, _) => { }
              // setting region_by_resel and resel_by_region happens at the start
            } // match expression to check contiguity. If contiguous, add to neighbors
          } // loop which checks adjacent resels for contiguity.
        } // loop over adjacent resels, updating region_by_resel and resels_by_region
      } // consider resel. if a new region, look for adjacent resels
    } // for each y
  } // for each x

  (region_by_resel, resels_by_region)
}

/// Given a reselboard and the mapping between regions and resels,
/// get the resel class at each region, plus dense class indices.
/// (The dense class indices are used when running.)
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

  (class_by_region, wire_nodes, input_nodes, output_nodes, logic_nodes)
}


#[derive(Debug, Clone)]
struct ResoCircuit {
  // aux drawing data
  image: image::DynamicImage,
  reselboard: Vec<Vec<Resel>>,
  region_by_resel: Vec<Vec<usize>>,
  resels_by_region: Vec<Vec<(usize, usize)>>,

  // index regions by resel class
  // in addition to region index, this also maintains a dense index for wire, io, logic
  // (e.g. region 7 might be wire 3, so wire_nodes[3] == 7)
  class_by_region: Vec<Resel>, // length == number of total regions
  wire_nodes: Vec<usize>,    // length == number of wire regions
  input_nodes: Vec<usize>,   // 
  output_nodes: Vec<usize>,
  logic_nodes: Vec<usize>,

  // connectivity data between classes
  // uses the dense indices for wire_nodes, input_nodes, etc. above
  // (e.g. we might have input_nodes[4] == 8, and input_to_wire[4] == [3,]
  //  which means region 8 is input 4, and has incident wire 3, which is region 7)
  // (But you can ignore region index here, since we dense per-class indices.
  input_to_wire: Vec<Vec<usize>>,  // input_idx -> wire_idx. (input nodes poll incident wires)
  input_to_logic: Vec<Vec<usize>>,
  input_to_output: Vec<Vec<usize>>,
  logic_to_output: Vec<Vec<usize>>,
  output_to_wire: Vec<Vec<usize>>,
  
  // temporary state data used at runtime
  wire_state: Vec<bool>, // length == number of wire regions
  logic_state: Vec<bool>,
  output_state: Vec<bool>,
}

/*
fn compile_reso_circuit_from_image(img: &image::DynamicImage) -> ResoCircuit {
  let reselboard = image_to_reselboard(img);

  let (width, height) = (reselboard.len(), reselboard[0].len());

  let (region_by_resel, resels_by_region) = resel_region_mapping_from_reselboard(
    &reselboard, width, height
  );

  
  let class_by_region = vec![Resel::Empty; 0];
    

  ResoCircuit {
    image: img.clone(),
    reselboard: reselboard,
    region_by_resel: region_by_resel,
    resels_by_region: resels_by_region,
    class_by_region: vec![Resel::Empty; 0],
    wire_nodes: vec![0; 0],
    input_nodes: vec![0; 0],
    output_nodes: vec![0; 0],
    logic_nodes: vec![0; 0],
    input_to_wire: vec![vec![0; 0]; 0],
    input_to_logic: vec![vec![0; 0]; 0],
    input_to_output: vec![vec![0; 0]; 0],
    logic_to_output: vec![vec![0; 0]; 0],
    output_to_wire: vec![vec![0; 0]; 0],
    wire_state: vec![false; 0],
    logic_state: vec![false; 0],
    output_state: vec![false; 0],
  };
}*/

// todo! next time lynn:
// 1. ResoCircuit struct, which is instantiated from a ReselBoard
// 2. Simple mappings for region indices. (class_by_region, wire_nodes, etc.)
// 3. With that done, adjacency mappings. (input_to_wire, etc.)
// 4. Iteration loop. (see readme)
// 5. Serialization, etc?
impl ResoCircuit {

  //fn from_reselboard(&self, reselboard: &Vec<Vec<Resel>) -> Self {
  //  let (width, height) = (reselboard.len(), reselboard[0].len());
  //}

  /*
  fn from_image(img: &image::DynamicImage) -> Self {
    let reselboard = image_to_reselboard(img);
    Self::from_reselboard(&reselboard)
  }

  fn from_filename(filename: &str) -> Self {
    let img = load_image_from_filename(filename);
    Self::from_image(&img)
  }
  */
}


fn main() {
  let img = load_image_from_filename("test.png");
  let (width, height) = img.dimensions();
  println!("Dimensions: {}x{}", width, height);
  println!("Reselboard: {:?}", image_to_reselboard(&img));
  let (region_by_resel, resels_by_region) = 
    resel_region_mapping_from_reselboard(&image_to_reselboard(&img), width as usize, height as usize);
  println!("Region by resel:\n{:?}", region_by_resel);
  println!("Resel by region:\n{:?}", resels_by_region);

  let (class_by_region, wire_nodes, input_nodes, output_nodes, logic_nodes) = 
    class_indices_from_reselboard_and_regions(
      &image_to_reselboard(&img),
      &region_by_resel,
      &resels_by_region,
    );
  println!("Class by region:\n{:?}", class_by_region);
  println!("Wire nodes:\n{:?}", wire_nodes);
  println!("Input nodes:\n{:?}", input_nodes);
  println!("Output nodes:\n{:?}", output_nodes);
  println!("Logic nodes:\n{:?}", logic_nodes);

}

