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


pub fn neighborhood(resel: Resel, x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
  // Given resel class + x,y + width,height, get the neighborhood of Resels from it

  // Todo: 
  // 1. Rename
  // 2. Refactor: Create `impl Resel { pub fn neighbor_deltas() -> (isize, isize)`
  // 3. Refactor: Create separate func for `map`? `deltas_to_absolute_neighbors`
  // 4. Fix?: Can overflow for huge x/y/width/height, or processors with very small usize/isize
  // 5. Continue! Complete our Region Mapper

  match resel {
    (
      Resel::WireOrangeOff   | Resel::WireOrangeOn   |
      Resel::WireSapphireOff | Resel::WireSapphireOn |
      Resel::WireLimeOff     | Resel::WireLimeOn
    ) => vec![
      (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1)
    ],
    (
      Resel::AND | Resel::XOR | Resel::Input | Resel::Output
    ) => vec![(1,0), (0,1), (-1,0), (0,-1)],
    _ => vec![]
  }
  .into_iter()
  .map(|(dx, dy)| 
    (
        ((x as isize + dx + width as isize) as usize % width),
        ((y as isize + dy + height as isize) as usize % height),
    )
  )
  .collect()
}

fn region_map_from_reselboard(
  board: &Vec<Vec<Resel>>
) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {
  let (width, height) = (reselboard.len(), reselboard[0].len());
  let mut visited:       Vec<Vec<bool>>  = vec![vec![false; height as usize]; width as usize];
  let mut region_idx:    usize = 0;
  let mut xy_to_region:  Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
  let mut region_to_xys: Vec<Vec<(usize, usize)>> = vec![Vec::new()];

  for x in 0..width { for y in 0..height { if !visited[x][y] {
    let color = board[x][y];
    if color == Resel::Empty {
      visited[x][y] = true;
      region_to_xys[0].push((x,y));
    } else {
      // New region! Set up our variables and explore
      region_idx += 1;
      region_to_xys(Vec::new());

      // Neighbors only holds unvisited Resels of the .same() color
      let mut neighbors: Vec<(usize, usize)> = vec![(x,y)];

      while !neighbors.is_empty() {
        // Record new pixel in our region
        let (x, y) = neighbors.pop().unwrap();
        xy_to_region[x][y] = region_idx;
        region_to_xys[region_idx].push((x,y));
        visited[x][y] = true;
        
        /* for dx, dy in neighborhood:
        todo!
        if board[x][y].same(color) && !visited[x][y] {

        }
        */
      } // For each surrounding neighbor, 
    } // Start recording a new region
  }}} // for each unvisited x, y
  (xy_to_region, region_to_xys)
}

/*
fn resel_region_mapping_from_reselboard(
  reselboard: &Vec<Vec<Resel>>
) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {
  // Returns xy_to_region, region_to_xys
  // possible runtime error if called with empty reselboard,
  // or reselboard with a column that is too short
  let (width, height) = (reselboard.len(), reselboard[0].len());
  let mut visited:       Vec<Vec<bool>>  = vec![vec![false; height as usize]; width as usize];
  let mut region_idx:    usize = 0;
  let mut xy_to_region:  Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
  let mut region_to_xys: Vec<Vec<(usize, usize)>> = vec![Vec::new()];

  // region_to_xys[0] empty-- we index regions starting with 1.
  // 'xy_to_region[x][y] = 0' means [x][y] doesn't have a region assignment
  region_to_xys.push(Vec::new());

  for x in 0..width {
    for y in 0..height {
      if visited[x][y] {
        // Already visited; pass
      } else if reselboard[x][y] == Resel::Empty {
        // Empty can not be a region. No other work to do here
        visited[x][y] = true
      } else {
        // Unvisited Resel marks a new region!
        // Update our region count, and prepare to mark new resels.
        // (Per above, region_idx 0 skipped so region_to_xys[0] stays empty.)
        // (On first loop, region_idx == 1, region_to_xys.len() == 2)
        region_idx += 1;
        region_to_xys.push(Vec::new());

        // Explore neighbors one-by-one, starting with our first resel at (x,y)
        let mut neighbors: Vec<(usize, usize)> = Vec::new();        
        neighbors.push((x, y));

        // `neighbors` is only empty once all regions have been explored.
        while !neighbors.is_empty() {
          let (x, y) = neighbors.pop().unwrap();
          // visiting a new resel!
          xy_to_region[x][y] = region_idx;
          region_to_xys[region_idx].push((x,y));
          visited[x][y] = true;
          // todo: should visited be set even if a resel neighbor is not of the same class?
          // as i read this, i think this is wrong, but the printed results are correct...
          // ... perhaps this is my food-addled brain simply not understanding things :)

          // Check contiguity to add neighbors in direction (dx, dy)
          for (dx, dy) in {
            // contiguity is orthogonal. wires can also be diagonally orthogonal
            if [
              Resel::WireOrangeOff,   Resel::WireOrangeOn,
              Resel::WireSapphireOff, Resel::WireSapphireOn,
              Resel::WireLimeOff,     Resel::WireLimeOn
            ].contains(&reselboard[x][y]) {
              // then iterate over diagonal and orthogonal neighbors
              [(1, 0),  (0, height-1), (width-1, 0),  (0, 1),
               (1, height-1), (width-1, height-1), (width-1,1), (1,1),
              ]
            } else if [
              Resel::AND, Resel::XOR, Resel::Input, Resel::Output
            ].contains(&reselboard[x][y]) {
              // then iterate over Orthogonal neighbors only. (dx,dy = 0,0 is skipped below.)
              [(1,0), (0,height-1), (width-1, 0), (0, 1), (0,0), (0,0), (0,0), (0,0)]
            } else { // No neighbors, dead case.
              [(0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)]
            }
          }.iter() { // for (dx, dy) in ..neighbors to check.. {
            // Check if neighbor is not already visited, and is not the originating pixel
            //println!("({},{}) Checking neighbor ({}, {})", x, y, (x + dx) % width, (y + dy)%height);
            if (
              (*dx != 0 || *dy != 0)
              && !visited[(x + dx) % width][(y + dy)%height]
              && is_resel_same_class(
                reselboard[x][y],
                reselboard[(x + dx) % width][(y + dy)%height]
              )
           ) {
              neighbors.push(((x + dx) % width, (y + dy)%height));
            } else {
              // neighbor coord is invalid, nothing to do!
              //println!("  ... {} {} {}", *dx != 0, *dy != 0, !visited[(x + dx) % width][(y + dy)%height]);
            } // If the same class, add to neighbors and mark as visited :)
          } // loop which checks adjacent resels for contiguity.
        } // loop over adjacent resels, updating xy_to_region and region_to_xys
      } // consider resel. If unvisited, it's a new region, so look for adjacent resels!
    } // for each x, y, if unvisited,
  } 
  // Read from bottom up!

  (xy_to_region, region_to_xys)
}
*/

#[cfg(test)]
mod reselboard_tests {
  use super::*;
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

}
