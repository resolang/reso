/*
regionmap.rs

Identify the regions in a Reselboard Vec<Vec<Resel>>.

Used with adjacencymap.rs to create a ResoCircuit

pub fn region_map_from_reselboard(
  board: &Vec<Vec<Resel>>
) -> RegionMap

pub struct RegionMap {
  xy_to_region: Vec<Vec<usize>>,            // [x][y] -> i
  region_to_xys: Vec<Vec<(usize, usize)>>,  // [i] -> [(x,y),...]
  region_to_resel:  Vec<Resel>,             // [Resel::Empty, Resel::And, ...]
  
  // dense class indices, for iterating over wires/inputs/logics/outputs
  wire_regions:     Vec<usize>,
  input_regions:    Vec<usize>,
  logic_regions:    Vec<usize>,
  output_regions:   Vec<usize>,
}

TODO:
- Make AdjacencyMap from RegionMap to build graph:
  wires -> inputs
  inputs -> logic
  inputs -> outputs
  logic -> outputs
  outputs -> wires

- Handle overflows in neighbor code; perhaps use `?`
- region mapper should probably return something like Result<Option<T>, E>
- Find some way to make generic and publish the CCL algorithm
*/
use crate::resel::{Resel};
use crate::reselboard::{get_neighbors, delta_to_neighbor}

pub struct RegionMap {
  xy_to_region: Vec<Vec<usize>>,            // [x][y] -> i
  region_to_xys: Vec<Vec<(usize, usize)>>,  // [i] -> [(x,y),...]
  region_to_resel:  Vec<Resel>,             // [Resel::Empty, Resel::And, ...]

  // dense class indices, for iterating over wires/inputs/logics/outputs
  wire_regions:     Vec<usize>,
  input_regions:    Vec<usize>,
  logic_regions:    Vec<usize>,
  output_regions:   Vec<usize>,
}

/*
let (
  xy_to_region, region_to_xys, region_to_resel,
  wire_regions, input_regions, logic_regions, output_regions
) = region_map_from_reselboard(board)
*/
/// Given a reselboard, find and index regions of adjacent elements.
/// Return as instance of RegionMap, which just holds all the useful data
/// 
pub fn region_map_from_reselboard(
  board: &Vec<Vec<Resel>>
) -> RegionMap {
  // todo: Vec<Vec<>> not necessarily grid. Check!
  let (width, height) = (board.len(), board[0].len());

  // visited and region_idx: Memory used only when compiling
  let mut visited:       Vec<Vec<bool>>  = vec![vec![false; height as usize]; width as usize];
  let mut region_idx:    usize = 0;
  
  // Region mapping data
  let mut xy_to_region:  Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
  let mut region_to_xys: Vec<Vec<(usize, usize)>> = vec![vec![]];
  let mut region_to_resel: Vec<Resel> = vec![Resel::Empty];

  // Dense class indices
  let mut wire_regions: Vec<usize> = vec![];
  let mut input_regions: Vec<usize> = vec![];
  let mut logic_regions: Vec<usize> = vec![];
  let mut output_regions: Vec<usize> = vec![];


  for x in 0..width { for y in 0..height { if !visited[x][y] {
    let resel = board[x][y];
    if resel == Resel::Empty {
      visited[x][y] = true;
      region_to_xys[0].push((x,y));
    } else {
      // New region! Set up our variables and explore
      region_idx += 1;
      region_to_xys.push(Vec::new());
      region_to_resel.push(resel);
      //println!("\nNew region {} with resel {:?}\nNeighbors:", region_idx, resel);
      
      if resel.is_wire()   { wire_regions.push(region_idx)   }
      if resel.is_input()  { input_regions.push(region_idx)  }
      if resel.is_logic()  { logic_regions.push(region_idx)  }
      if resel.is_output() { output_regions.push(region_idx) }

      // Neighbors only holds unvisited Resels of the .same() color
      let mut neighbors: Vec<(usize, usize)> = vec![(x,y)];

      while !neighbors.is_empty() {
        // Record new pixel in our region
        let (x, y) = neighbors.pop().unwrap();
        xy_to_region[x][y] = region_idx;
        region_to_xys[region_idx].push((x,y));
        visited[x][y] = true;

        
        for (nx, ny) in get_neighbors(resel, x, y, width, height) {
          if board[nx][ny].same(resel) && !visited[nx][ny] {
            // Only add unvisited neighbor coordinates of the same class
            neighbors.push((nx, ny));
            visited[nx][ny] = true;
            //println!("... ({},{}) sees ({},{})", x,y, nx, ny);

          } // If unvisited & same class, add to queue
        } // ... and check each surrounding neighbor.
      } // For each queued neighbor, record it... 
    } // Start recording a new region!
  }}} // for each x, y, if unvisited,
  // Returns
  RegionMap {
    xy_to_region,
    region_to_xys,
    region_to_resel,
    wire_regions,
    input_regions,
    logic_regions,
    output_regions
  }
}


#[cfg(test)]
mod reselboard_tests {
  use super::*;
  use std::collections::HashSet;
  use crate::reselboard::{
    load_image_from_filename,
    image_to_vecvecresel
  };



  #[test]
  fn test_regon_map_basic() {
    for board in [
      vec![vec![Resel::Empty]],
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_01_new-palette.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_02_new-palette_1.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_02_new-palette.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_03_01.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_03_02.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_03_03.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_03_04.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_03_alloff.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_03_allon.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_04.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_05_01.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_05_02.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_05_03.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_05_04.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_05_05.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_05_06.png").unwrap()),
      image_to_vecvecresel(&load_image_from_filename("./src/testing/test_06.png").unwrap()),
    ] {
      let rm = region_map_from_reselboard(&board);

  
      let (width, height) = (board.len(), board[0].len());
      let N_regions = rm.region_to_xys.len();
      let mut accounted_xy:       Vec<Vec<bool>>  = vec![vec![false; height as usize]; width as usize];
      let mut accounted_region: Vec<bool> = vec![false; N_regions];

      assert!(N_regions >= 1);

      for region_idx in 0..N_regions {
        let resel_by_region = rm.region_to_resel[region_idx];

        for (x,y) in &rm.region_to_xys[region_idx] {
          // Assert all elements in the region are the same color
          let resel_by_coord = board[*x][*y];
          assert!(resel_by_coord.same(resel_by_region));

          // Account each x,y
          assert!(!accounted_xy[*x][*y]);
          accounted_xy[*x][*y] = true;

          // Check xy_to_region is consistent
          assert_eq!(rm.xy_to_region[*x][*y], region_idx);       
        }
      }

      // Now, each `x,y` should be accounted for
      for x in 0..width{ for y in 0..height {
        assert!(accounted_xy[x][y]);
      }}

      // Account for each region_idx in the dense indices
      for region_iterator in [
        vec![0], rm.wire_regions, rm.input_regions, rm.logic_regions, rm.output_regions
      ] {
        for region_idx in region_iterator {
          assert!(!accounted_region[region_idx]);
          accounted_region[region_idx] = true;
          }
        }
      
      // Now, each region_idx should be accounted for
      for region_idx in 0..N_regions {
        assert!(accounted_region[region_idx])
      }

      // TODO: How is it that duplicates in `region_to_xys` skipped the tsts
    }    
  }


  #[test]
  fn test_region_map_01() {
    /*
    This is a fragile test.
    It assumes an order to the elements returned, despite 
    region_map_from_reselboard not guaranteeing such an order.

    Someone better at Rust should rework this.
    */
    let board = image_to_vecvecresel(
      &load_image_from_filename(
        "./src/testing/test_01_new-palette.png"
      ).unwrap()
    );

    let rm = region_map_from_reselboard(&board);

    assert_eq!(
      rm.xy_to_region,
      vec![vec![1,2,3,0,1], vec![4,1,3,3,0], vec![0,5,3,3,1]]
    );

    assert_eq!(
      rm.region_to_xys,
      vec![
        vec![(0,3), (1,4), (2,0)],
        vec![(0,0), (0,4), (2,4), (1,1)],
        vec![(0,1)],
        vec![(0,2),(2,2),(2,3),(1,3),(1,2)],
        vec![(1,0)],
        vec![(2,1)]
      ]
    );

    assert_eq!(
      rm.region_to_resel,
      vec![
        Resel::Empty,
        Resel::WireOrangeOn, Resel::WireLimeOn, Resel::WireSapphireOff,
        Resel::Input, Resel::Input
      ]
    );

    assert_eq!(rm.wire_regions,   vec![1,2,3]);
    assert_eq!(rm.input_regions,  vec![4,5]);
    assert_eq!(rm.logic_regions,  vec![]);
    assert_eq!(rm.output_regions, vec![]);
  }

  #[test]
  fn test_region_map_06() {
    /*
    This is a fragile test.
    It assumes an order to the elements returned, despite 
    region_map_from_reselboard not guaranteeing such an order.

    Someone better at Rust should rework this.
    */
    let board = image_to_vecvecresel(
      &load_image_from_filename(
        "./src/testing/test_06.png"
      ).unwrap()
    );

    let rm = region_map_from_reselboard(&board);

    assert_eq!(
      rm.xy_to_region,
      vec![vec![1,0,2], vec![1,3,3], vec![1,4,2], vec![1,2,5], vec![2,2,5]]
    );

    assert_eq!(
      rm.region_to_xys,
      vec![
        vec![(0,1)],
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(0,2), (4,1), (3,1), (2,2), (4,0)],
        vec![(1,1), (1,2)],
        vec![(2,1)],
        vec![(3,2), (4,2)]
      ]
    );

    assert_eq!(
      rm.region_to_resel,
      vec![
        Resel::Empty,
        Resel::WireLimeOn, Resel::WireOrangeOff,
        Resel::WireSapphireOn,
        Resel::Input, Resel::Input
      ]
    );

    assert_eq!(rm.wire_regions,   vec![1,2,3]);
    assert_eq!(rm.input_regions,  vec![4,5]);
    assert_eq!(rm.logic_regions,  vec![]);
    assert_eq!(rm.output_regions, vec![]);
  }
}

// eof