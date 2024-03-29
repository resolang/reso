//! regionmap.rs -- Identify the contiguous regions in a ReselBoard Vec<Vec<Resel>.
//! 
//! Exactly what it says on the tin, with two complications:
//! 1. Resels have 4-neighborhood connectivity, except wires, which have 8.
//! 2. On and off wires of the same color (e.g. orange) are the same.
//! 
//! Key to a RegionMap are region indices. Region 0 corresponds to any 'empty'
//! Resel. All others start counting from 1.
//! 
//! For performance, we also maintain implicit list of "dense" class indices
//! for input resels, output resels, wire resels, and logic resels.
//! 
//! TODO:
//! - Example code in docs
//! - Region with mixed on/off wires should be classed as "on".
//! - add `width`, `height` to `RegionMap`? Or even a whole `ReselBoard`?
//! - Ensure sorted ordering on all outputs?
//! 
//! - region mapper should probably return something like Result<Option<T>, E>
//! - Consider: This implements connected component labeling. Publish a generic version?

use crate::resel::{Resel};
use crate::reselboard::{
  ReselBoard,
  get_neighbors
};

/// RegionMap -- Mapping between contiguous regions and their coordinates.
/// 
/// A region is just a contiguous blob of resels. Regions can be one resel
/// large (and should be for most non-wire resels.)
/// 
/// Region index starts at 1 (with '0' reserved for the empty region).
/// 
/// - `xy_to_region[x][y]` = the region index at a given coordinate
/// - `region_to_xys[i]`   = the coordinate of a region index
/// - `region_to_resel[i]` = the Resel of a given region
/// - The "dense indices" maintain a list of region indices
///   four different types of resel.
///   - `wire_regions` for wires (Orange, Sapphire, Lime, on or off)
///   - `input_regions` for any input region
///   - `output_regions` for any output region
///   - `logic_regions` for And and Xor regions
/// - The `reverse_dense` index gives you the dense index value for any `region_index`.
///   - This is hard to wrap your mind around; look at the tests for examples.
#[derive(Debug, Clone)]
pub struct RegionMap {
  pub xy_to_region: Vec<Vec<usize>>,            // [x][y] -> i
  pub width: usize,
  pub height: usize,

  pub region_to_xys: Vec<Vec<(usize, usize)>>,  // [i] -> [(x,y),...]
  pub region_to_resel:  Vec<Resel>,             // [Resel::Empty, Resel::And, ...]

  // dense class indices, for iterating over wires/inputs/logics/outputs
  // list of region indices; position in list is an inherent "dense index"
  // todo: Can these four items be made as one structure?
  pub wire_regions:     Vec<usize>,
  pub input_regions:    Vec<usize>,
  pub logic_regions:    Vec<usize>,
  pub output_regions:   Vec<usize>,

  /* reverse dense index
  e.g. given region_index ri, what is the dense index?
  assume region_to_resel(ri).is_wire()
  
  O(n): wire_regions.iter().position(|&wire_ri| wire_ri == ri)
  O(1): reverse_dense[ri]
  */
  pub reverse_dense: Vec<usize>

  
}

impl RegionMap {
  /// For any region index, get the list of region indices of adjacent regions
  pub fn get_adjacent_regions(&self, region: usize) -> Vec<usize> {
    let mut adjacent_regions = vec![];

    for (x,y) in &self.region_to_xys[region] {
      for (nx, ny) in get_neighbors(
        vec![(1,0),(0,1),(-1,0),(0,-1)], // adjacencies are only orthogonal, wire or not
        *x, *y, self.width, self.height
      ) {
        let neighbor_region = self.xy_to_region[nx][ny];
        if (
          region != neighbor_region
          && !adjacent_regions.contains(&neighbor_region)
        ) {
          adjacent_regions.push(neighbor_region)
        }
      }
    }

    adjacent_regions.sort();
    adjacent_regions
  }
}


/// Given a reselboard, find and index regions of adjacent elements.
/// Return as instance of RegionMap, which holds all the useful data.
fn region_map_from_reselboard(
  rb: &ReselBoard,
) -> RegionMap {
  // todo: Vec<Vec<>> not necessarily grid. Check!
  let (width, height) = (rb.width, rb.height);

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

  // Reverse dense index ties region_index and the dense indices above
  let mut reverse_dense: Vec<usize> = vec![0];


  for x in 0..width { for y in 0..height { if !visited[x][y] {
    let resel = rb.board[x][y];
    if resel == Resel::Empty {
      visited[x][y] = true;
      region_to_xys[0].push((x,y));
    } else {
      // New region! Set up our variables and explore
      region_idx += 1;
      region_to_xys.push(Vec::new());
      region_to_resel.push(resel);
      //println!("\nNew region {} with resel {:?}\nNeighbors:", region_idx, resel);
      
      // Set up dense and reverse-dense index
      if resel.is_wire()   {
        reverse_dense.push(wire_regions.len());
        wire_regions.push(region_idx);
      }
      if resel.is_input()  {
        reverse_dense.push(input_regions.len());
        input_regions.push(region_idx)
      }
      if resel.is_logic()  {
        reverse_dense.push(logic_regions.len());
        logic_regions.push(region_idx)
      }
      if resel.is_output() {
        reverse_dense.push(output_regions.len());
        output_regions.push(region_idx)
      }

      // Neighbors only holds unvisited Resels of the .same() color
      let mut neighbors: Vec<(usize, usize)> = vec![(x,y)];

      while !neighbors.is_empty() {
        // Record new pixel in our region
        let (x, y) = neighbors.pop().unwrap();
        xy_to_region[x][y] = region_idx;
        region_to_xys[region_idx].push((x,y));
        visited[x][y] = true;

        
        for (nx, ny) in rb.get_neighbors(x, y) {
          if rb.board[nx][ny].same(resel) && !visited[nx][ny] {
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
    width,
    height,
    region_to_xys,
    region_to_resel,
    wire_regions,
    input_regions,
    logic_regions,
    output_regions,
    reverse_dense
  }
}

impl From<ReselBoard> for RegionMap {
  /// rm = RegionMap::from(rb.clone());
  fn from (rb: ReselBoard) -> RegionMap {
    region_map_from_reselboard(&rb)
  }
}

impl From<&ReselBoard> for RegionMap {
  /// rm = RegionMap::from(rb.clone());
  fn from (rb: &ReselBoard) -> RegionMap {
    region_map_from_reselboard(rb)
  }
}

// todo: Consider `impl From` from filename, image, vec str

#[cfg(test)]
mod reselboard_tests {
  use super::*;
  use crate::reselboard::{
    load_image_from_filename,
  };

  #[test]
  fn test_regon_map_basic() {
    for rb in [
      ReselBoard::from(vec![vec![Resel::Empty]]),
      ReselBoard::from(load_image_from_filename("./src/testing/test_01_new-palette.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_02_new-palette_1.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_02_new-palette.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_03_01.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_03_02.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_03_03.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_03_04.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_03_alloff.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_03_allon.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_04.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_05_01.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_05_02.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_05_03.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_05_04.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_05_05.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_05_06.png").unwrap()),
      ReselBoard::from(load_image_from_filename("./src/testing/test_06.png").unwrap()),
    ] {
      let rm = RegionMap::from(&rb);//region_map_from_reselboard(&rb);

      let (width, height) = (rb.board.len(), rb.board[0].len());
      let n_regions = rm.region_to_xys.len();
      let mut accounted_xy:       Vec<Vec<bool>>  = vec![vec![false; height as usize]; width as usize];
      let mut accounted_region: Vec<bool> = vec![false; n_regions];

      assert!(n_regions >= 1);

      for region_idx in 0..n_regions {
        let resel_by_region = rm.region_to_resel[region_idx];

        for (x,y) in &rm.region_to_xys[region_idx] {
          // Assert all elements in the region are the same color
          let resel_by_coord = rb.board[*x][*y];
          assert!(resel_by_coord.same(resel_by_region));

          // Account each x,y
          assert!(!accounted_xy[*x][*y]);
          accounted_xy[*x][*y] = true;

          // Check xy_to_region is consistent
          assert_eq!(rm.xy_to_region[*x][*y], region_idx);      

          // Check reverse_dense is consistent
          // ri == xxxx_regions[reverse_dense[ri]]
          if !resel_by_region.is_empty() {
            assert_eq!(
              region_idx,
              { // Get appropriate dense index
                if resel_by_region.is_wire()   { &rm.wire_regions } else
                if resel_by_region.is_input()  { &rm.input_regions } else
                if resel_by_region.is_logic()  { &rm.logic_regions } else
                if resel_by_region.is_output() { &rm.output_regions } else
                { panic!("This should not be possible to reach!") }
              }[rm.reverse_dense[region_idx]]
            )
          }
        }
      }

      // Now, each `x,y` should be accounted for
      for x in 0..width{ for y in 0..height {
        assert!(accounted_xy[x][y]);
      }}

      // Account for each region_idx in the dense indices
      for region_iterator in [
        vec![0],
        rm.wire_regions.clone(),
        rm.input_regions.clone(),
        rm.logic_regions.clone(),
        rm.output_regions.clone()
      ] {
        for region_idx in region_iterator {
          assert!(!accounted_region[region_idx]);
          accounted_region[region_idx] = true;
          }
        }
      
      // Now, each region_idx should be accounted for
      for region_idx in 0..n_regions {
        assert!(accounted_region[region_idx])
      }
      
      // Test undirected adjacency
      for region_idx in 0..n_regions {
        for rj in rm.get_adjacent_regions(region_idx) {
          assert!(
            rm.get_adjacent_regions(rj).contains(&region_idx)
          );
        }
      }
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
    let rb = ReselBoard::from(
      load_image_from_filename(
        "./src/testing/test_01_new-palette.png"
      ).unwrap()
    );

    let rm = RegionMap::from(&rb);

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
    assert_eq!(rm.reverse_dense,  vec![0,0,1,2,0,1]);

    // test get adjacent regions

    assert_eq!(
      rm.get_adjacent_regions(0),
      vec![1,3,4,5]
    );
    assert_eq!(
      rm.get_adjacent_regions(1),
      vec![0,2,3,4,5]
    );
    assert_eq!(
      rm.get_adjacent_regions(2),
      vec![1,3,5]
    );
    assert_eq!(
      rm.get_adjacent_regions(3),
      vec![0,1,2,5]
    );
    assert_eq!(
      rm.get_adjacent_regions(4),
      vec![0,1]
    );
    assert_eq!(
      rm.get_adjacent_regions(5),
      vec![0,1,2,3]
    );
  }

  #[test]
  fn test_region_map_06() {
    /*
    This is a fragile test.
    It assumes an order to the elements returned, despite 
    region_map_from_reselboard not guaranteeing such an order.

    Someone better at Rust should rework this.
    */
    let rb = ReselBoard::from(
      load_image_from_filename(
        "./src/testing/test_06.png"
      ).unwrap()
    );

    let rm = RegionMap::from(&rb);

    assert_eq!(
      rm.xy_to_region,
      vec![
        vec![1,0,2],
        vec![1,3,3],
        vec![1,4,2],
        vec![1,2,5],
        vec![2,2,5]
      ]
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
        Resel::WireLimeOn,
        Resel::WireOrangeOff,
        Resel::WireSapphireOn,
        Resel::Input,
        Resel::Input
      ]
    );

    // ... huh. these ended up with the exact same dense indices
    assert_eq!(rm.wire_regions,   vec![1,2,3]);
    assert_eq!(rm.input_regions,  vec![4,5]);
    assert_eq!(rm.logic_regions,  vec![]);
    assert_eq!(rm.output_regions, vec![]);
    assert_eq!(rm.reverse_dense,  vec![0,0,1,2,0,1]);
    
    // test get adjacent regions
    assert_eq!(
      rm.get_adjacent_regions(0),
      vec![1,2,3,]
    );
    assert_eq!(
      rm.get_adjacent_regions(1),
      vec![0,2,3,4,5,]
    );
    assert_eq!(
      rm.get_adjacent_regions(2),
      vec![0,1,3,4,5,]
    );
    assert_eq!(
      rm.get_adjacent_regions(3),
      vec![0,1,2,4,]
    );
    assert_eq!(
      rm.get_adjacent_regions(4),
      vec![1,2,3,]
    );
    assert_eq!(
      rm.get_adjacent_regions(5),
      vec![1,2,]
    );
  }

  #[test]
  fn test_region_map_half_adder() {
    /*
    This is a fragile test.
    It assumes an order to the elements returned, despite 
    region_map_from_reselboard not guaranteeing such an order.

    Someone better at Rust should rework this.
    */
    let rb = ReselBoard::from(
      load_image_from_filename(
        "./src/testing/test_half_adder.png"
      ).unwrap()
    );

    let rm = RegionMap::from(&rb);

    assert_eq!(
      rm.xy_to_region,
      vec![
        vec![0,0,1,2,0,0],
        vec![0,0,1,2,0,0],
        vec![0,0,1,2,0,0],
        vec![0,0,3,3,0,0],
        vec![0,4,5,6,7,0],
        vec![0,8,0,0,9,0],
        vec![0,8,0,0,9,0],
        vec![0,8,0,0,9,0],
      ]
    );

    // skip region 0 because there are so many
    for (region_idx, expected) in [
      (1, vec![(0,2),(1,2),(2,2)]),
      (2, vec![(0,3),(1,3),(2,3)]),
      (3, vec![(3,2),(3,3)]),
      (4, vec![(4,1)]),
      (5, vec![(4,2)]),
      (6, vec![(4,3)]),
      (7, vec![(4,4)]),
      (8, vec![(5,1),(6,1),(7,1)]),
      (9, vec![(5,4),(6,4),(7,4)]),
    ] {
      assert_eq!(
        rm.region_to_xys[region_idx],
        expected
      )
    }

    assert_eq!(
      rm.region_to_resel,
      vec![
        Resel::Empty,
        Resel::WireOrangeOff,
        Resel::WireSapphireOff,
        Resel::Input,
        Resel::Output,
        Resel::XOR,
        Resel::AND,
        Resel::Output,
        Resel::WireLimeOff,
        Resel::WireLimeOff
      ]
    );

    assert_eq!(rm.wire_regions,   vec![1,2,8,9]);
    assert_eq!(rm.input_regions,  vec![3,]);
    assert_eq!(rm.logic_regions,  vec![5,6]);
    assert_eq!(rm.output_regions, vec![4,7]);
    assert_eq!(rm.reverse_dense,  vec![0,0,1,0,0,0,1,1,2,3]);
    
    // test get adjacent regions
    assert_eq!(
      rm.get_adjacent_regions(0),
      vec![1,2,3,4,5,6,7,8,9]
    );
    assert_eq!(
      rm.get_adjacent_regions(1),
      vec![0,2,3,]
    );
    assert_eq!(
      rm.get_adjacent_regions(2),
      vec![0,1,3,]
    );
    assert_eq!(
      rm.get_adjacent_regions(3),
      vec![0,1,2,5,6]
    );
    assert_eq!(
      rm.get_adjacent_regions(4),
      vec![0,5,8]
    );
    assert_eq!(
      rm.get_adjacent_regions(5),
      vec![0,3,4,6],
    );
    assert_eq!(
      rm.get_adjacent_regions(6),
      vec![0,3,5,7],
    );
    assert_eq!(
      rm.get_adjacent_regions(7),
      vec![0,6,9],
    );
    assert_eq!(
      rm.get_adjacent_regions(8),
      vec![0,4,],
    );
    assert_eq!(
      rm.get_adjacent_regions(9),
      vec![0,7,],
    );
  }

  // todo: We could use more tests for more examples.
  // todo: The above tests could be made more robust; too fragile to ordering
}

// eof