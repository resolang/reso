use crate::resel::{Resel};
use image::{Rgba, DynamicImage, GenericImageView};

struct ReselBoard {
  board: Vec<Vec<Resel>>,
  image: DynamicImage,
}

// Helper function
pub fn load_image_from_filename(filename: &str) -> Option<DynamicImage> {
  match image::open(filename) {
    Ok(img) => Some(img),
    Err(_)  => None
  }
}

// Instantiate Vec<Vec<Resel>> 
pub fn image_to_vecvecresel(img: &DynamicImage) -> Vec<Vec<Resel>> {
  let (width, height) = img.dimensions();
  let mut reselboard = vec![vec![Resel::Empty; height as usize]; width as usize];
  for x in 0..width {
    for y in 0..height {
      let pixel = img.get_pixel(x, y);
      let resel = Resel::from(pixel);
      reselboard[x as usize][y as usize] = resel;
    }
  }
  reselboard
}


// delta_to_neighbor(x, y, dx, dy, width, height, wrap)
// returns Some (x+dx, y+dy), None if out of bounds
fn delta_to_neighbor(
  x: usize, y: usize,
  dx: isize, dy: isize,
  width: usize, height: usize,
  wrap: bool
) -> Option<(usize, usize)> {
  // todo: handle overflows, write tests for that
  let ax = x as isize + dx;
  let ay = y as isize + dy;
  if wrap { // wrap: No "out-of-bounds" to consider, just return (x+dx)%width
    Some(
      (
        ((ax + width as isize) as usize % width),
        ((ay + height as isize) as usize % height),
      )
    )
  } else { // No wrap; check (ax,ay) within bounds
    if (ax >= width as isize) || (ax < 0) || (ay >= height as isize) || (ay < 0)
    { // Out of bounds, return None
      None
    } else { // Within bounds, just return (ax, ay)
      Some((ax as usize, ay as usize))
    }
  }
}

// get_neighbors(resel, x, y, width, height) -> convenient list of coordinates
pub fn get_neighbors(
  resel: Resel, x: usize, y: usize, width: usize, height: usize
) -> Vec<(usize, usize)> {
  // Given resel class + x,y + width,height, get the neighborhood of (x,y) coordinates

  resel.delta_neighbors()
    .into_iter()
    .filter_map(|(dx, dy)| delta_to_neighbor(x, y, dx, dy, width, height, true))
    .collect()
}

/// Given a reselboard, find and index regions of adjacent elements.
/// Returns tuple (xy_to_region[x][y]->i, region_to_xys[i]->[(x,y), ...])
fn region_map_from_reselboard(
  board: &Vec<Vec<Resel>>
) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {
  // todo: Vec<Vec<>> not necessarily grid.
  let (width, height) = (board.len(), board[0].len());

  let mut visited:       Vec<Vec<bool>>  = vec![vec![false; height as usize]; width as usize];
  let mut region_idx:    usize = 0;
  let mut xy_to_region:  Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
  let mut region_to_xys: Vec<Vec<(usize, usize)>> = vec![vec![]];

  for x in 0..width { for y in 0..height { if !visited[x][y] {
    let resel = board[x][y];
    if resel == Resel::Empty {
      visited[x][y] = true;
      region_to_xys[0].push((x,y));
    } else {
      // New region! Set up our variables and explore
      region_idx += 1;
      region_to_xys.push(Vec::new());

      // Neighbors only holds unvisited Resels of the .same() color
      let mut neighbors: Vec<(usize, usize)> = vec![(x,y)];

      while !neighbors.is_empty() {
        // Record new pixel in our region
        let (x, y) = neighbors.pop().unwrap();
        xy_to_region[x][y] = region_idx;
        region_to_xys[region_idx].push((x,y));
        visited[x][y] = true;
        
        for (nx, ny) in get_neighbors(resel, x, y, width, height) {
          if board[x][y].same(resel) && !visited[x][y] {
            neighbors.push((nx, ny))
          } // If unvisited & same class, add to queue
        } // ... and check each surrounding neighbor.
      } // For each queued neighbor, record it... 
    } // Start recording a new region!
  }}} // for each x, y, if unvisited,
  (xy_to_region, region_to_xys)
}

#[cfg(test)]
mod reselboard_tests {
  use super::*;
  use std::collections::HashSet;

  #[test]
  fn load_image_doesnt_exist() {
    assert!(load_image_from_filename("this_does_not_exist.png").is_none())
    // assert None
  }

  #[test]
  fn load_image_does_exist() {
    assert!(load_image_from_filename("./src/testing/test_01_new-palette.png").is_some())
  }

  #[test]
  fn load_and_convert_image_test_01() {
    let img = load_image_from_filename("./src/testing/test_01_new-palette.png").unwrap();
    let board = image_to_vecvecresel(&img);

    for ((x, y), resel) in [
      ((0,0), Resel::WireOrangeOn),
      ((1,0), Resel::Input),
      ((2,0), Resel::Empty),
      ((0,1), Resel::WireLimeOn),
      ((1,1), Resel::WireOrangeOn),
      ((2,1), Resel::Input),
      ((0,2), Resel::WireSapphireOff),
      ((1,2), Resel::WireSapphireOff),
      ((2,2), Resel::WireSapphireOff),
      ((0,3), Resel::Empty),
    ] {
      // Assert pixel-to-resel == resel
      assert_eq!(Resel::from(img.get_pixel(x,y)), resel);
      if resel != Resel::Empty {
        // Assert resel-to-pixel == pixel
        // Ignore empty, because empty-to-pixel has many possibilities
        assert_eq!(img.get_pixel(x,y), <Rgba<u8>>::from(resel));
      }

      // Also check the board conversion is correct
      assert_eq!(board[x as usize][y as usize], resel)
    }
  }

  #[test]
  fn test_delta_to_neighbor() {
    for (x, y, dx, dy, width, height, wrap, expected) in [
      // Base case
      (0, 0, 0, 0, 1, 1, true, Some((0, 0))),
      // Generic cases
      (11, 11,  1,  0, 100, 100, true, Some((12,11))),
      (11, 11,  0,  0, 100, 100, true, Some((11,11))),
      (11, 11, -1, -1, 100, 100, true, Some((10,10))),
      (11, 11, -1,  1, 100, 100, true, Some((10,12))),
      (11, 11,  1,  0, 100, 100, false, Some((12,11))), // Repeat but with wrap=false
      (11, 11,  0,  0, 100, 100, false, Some((11,11))),
      (11, 11, -1, -1, 100, 100, false, Some((10,10))),
      (11, 11, -1,  1, 100, 100, false, Some((10,12))),
      // Cases forcing a wrap (from each border/corner)
      (0, 0, -1,  0, 100, 100, true, Some((99,0))),
      (0, 0, -1, -1, 100, 100, true, Some((99,99))),
      (0, 0,  0, -1, 100, 100, true, Some((0,99))),

      (99, 0,  1,  0, 100, 100, true, Some((0,0))),
      (99, 0,  1, -1, 100, 100, true, Some((0,99))),
      (99, 0,  0, -1, 100, 100, true, Some((99,99))),

      (0, 99, -1,  0, 100, 100, true, Some((99,99))),
      (0, 99, -1,  1, 100, 100, true, Some((99,0))),
      (0, 99,  0,  1, 100, 100, true, Some((0,0))),

      (99, 99,  1,  0, 100, 100, true, Some((0,99))),
      (99, 99,  1,  1, 100, 100, true, Some((0,0))),
      (99, 99,  0,  1, 100, 100, true, Some((99,0))),

      // Cases forcing a wrap but wrap=false
      (0, 0, -1,  0, 100, 100, false, None),
      (0, 0, -1, -1, 100, 100, false, None),
      (0, 0,  0, -1, 100, 100, false, None),

      (99, 0,  1,  0, 100, 100, false, None),
      (99, 0,  1, -1, 100, 100, false, None),
      (99, 0,  0, -1, 100, 100, false, None),

      (0, 99, -1,  0, 100, 100, false, None),
      (0, 99, -1,  1, 100, 100, false, None),
      (0, 99,  0,  1, 100, 100, false, None),

      (99, 99,  1,  0, 100, 100, false, None),
      (99, 99,  1,  1, 100, 100, false, None),
      (99, 99,  0,  1, 100, 100, false, None),

      // Consider dx, dy > 1
    ] {
      assert_eq!(
        delta_to_neighbor(x,y,dx,dy,width,height,wrap),
        expected
      )

      // Not tested: Overflow during isize/usize conversion
    }
  }

  #[test]
  fn test_get_neighbors() {
    // get_neighbors(resel: Resel, x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)>
    for (resel, x, y, width, height, neighbors) in [
      (Resel::Empty, 0, 0, 1, 1, Vec::<(usize, usize)>::new()),
      // Test non-wire neighborhoods within bounds
      (Resel::AND, 4, 4, 10, 10, vec![(4,5), (5,4), (3,4), (4,3)]),
      // Test wire neighborhoods within bounds
      (Resel::WireOrangeOn, 4, 4, 10, 10, vec![(4,5), (5,4), (3,4), (4,3), (5,5), (5,3), (3,5), (3,3)]),
      // Test non-wire neighborhoods near bounds
      (Resel::Input, 0, 4, 10, 10, vec![(0,5), (1,4), (9,4), (0,3)]),
      // Test wire neighborhoods near bounds
      (Resel::WireLimeOff, 4, 0, 10, 10, vec![(4,1), (5,0), (3,0), (4,9), (5,1), (5,9), (3,1), (3,9)]),
    ] {
      let neighbors_1: HashSet<(usize, usize)> = get_neighbors(resel, x, y, width, height).into_iter().collect();
      let neighbors_2: HashSet<(usize, usize)> = neighbors.into_iter().collect();
      assert_eq!(neighbors_1, neighbors_2);
    }
  }
  
  #[test]
  fn test_regon_map_basic() {
    for board in [
      vec![vec![Resel::Empty]],
    ] {

      let (xy_to_region, region_to_xys) = region_map_from_reselboard(&board);
      // Can't guarantee same structure of `expected`
      // Instead test:
      // 1. Same:    Each region has all the .same() colors
      // 2. Account: Each `xy` is accounted for exactly once across `region_to_xys`
      // 3 ... 
  
      let (width, height) = (board.len(), board[0].len());
      let mut accounted:       Vec<Vec<bool>>  = vec![vec![false; height as usize]; width as usize];

      for region_idx in 0..region_to_xys.len() {
        // Check that all board[x][y] are the same for all region_to_xys[region_idx]
        // using `iter()`, etc
        region_color = 
        /*
        assert!(
          region_to_xys[region_idx]
          .iter()
          .all(|(x, y)| board[*x][*y].same(board[region_to_xys[region_idx][0].0][region_to_xys[region_idx][0].1])));


        for (x, y) in &region_to_xys[region_idx] {
          // 
          
          
          // Accounting: Each `x,y` must be accounted for exactly once
          assert!(!accounted[*x][*y]);
          accounted[*x][*y] = true;

        }
        */
      }
    }    
  }

}

/*
TODOs:
- Basic tests for RegionMapper
- Test RegionMapper against expected values
- Rework into `impl reselboard`?
- Enforce Vec<Vec<Resel>> grid shape / fail if it doesn't have it?
- RegionMapper: Also report:
  - Dense class indices (wire_regions, input_regions, output_regions, logic_regions)
  - region_to_resel
- Go back and use magic ? to handle overflows?
*/