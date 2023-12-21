//! reselboard.rs: A grid of Resels
//! 
//! ReselBoard wraps a Vec<Vec<Resel>> and provides relevant operations to loading/reading.
//! 
//! If a `resel` is a `pixel`, then a ReselBoard is an image.
//! 
//! Example:
//! 
//! ```rust
//! let reselboard = image_to_reselboard(
//!   load_image_from_filename("some_file.png").unwrap()
//! );
//! 
//! ```


//!TODOs:
//!- ReselBoard Froms
//!- ReselBoard: Rethink how to handle image, text.
//! 
//!- impl `reselboard.set_resel(resel, x, y)`, updates pixel too
//!- impl `reselboard.set_pixel(Rgba, x, y)`, updates resel too
//!- Cleanup:
//!  - Fix this doc
//!  - Re-order / rename functions below
//!- Handle overflows in neighbor code; perhaps use `?`
//!- Ensure sorted ordering on outputs?
//!- Examples


use crate::resel::{Resel};
use image::{Rgba, DynamicImage, GenericImageView};

/// Utility over Vec<Vec<Resel>>, i.e. grid of Resel
#[derive(Clone, Debug)]
pub struct ReselBoard {
  pub board: Vec<Vec<Resel>>,
  pub image: Option<DynamicImage>,
  pub width: usize,
  pub height: usize
}

/// Consume an image and return a ReselBoard
fn image_to_reselboard(image: DynamicImage) -> ReselBoard {
  let (width, height) = image.dimensions();

  ReselBoard {
    board: image_to_vecvecresel(&image),
    image: Some(image),
    width: width as usize,
    height: height as usize,
  }
}

/// Consume a Vec<Vec<Resel>> and return a ReselBoard
/// (todo: optionally instantiate ReselBoard.image along with this)
fn vecvecresel_to_reselboard(board: Vec<Vec<Resel>>) -> ReselBoard {
  let width = board.len();
  let height = board[0].len();
  // todo: check board is a grid, at least one whole pixel
  ReselBoard {
    board: board,
    image: None, // todo: Optionally generate from ReselBoard
    width: width,
    height: height
  }
}

/// Helper function to load images
pub fn load_image_from_filename(filename: &str) -> Option<DynamicImage> {
  match image::open(filename) {
    Ok(img) => Some(img),
    Err(_)  => None
  }
}

/// Instantiate Vec<Vec<Resel>> from &DynamicImage
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

impl From<DynamicImage> for ReselBoard {
  fn from(image: DynamicImage) -> Self {
    image_to_reselboard(image)
  }
}

impl From<Vec<Vec<Resel>>> for ReselBoard {
  fn from(board: Vec<Vec<Resel>>) -> Self {
    vecvecresel_to_reselboard(board)
  }
}

impl ReselBoard {
  /// For a given (x,y) coordinate, return the absolute neighbor coordinates
  /// Wraps around the width and height of the board, and takes into account
  /// the Resel-specific neighborhoods. (8 for wires, 4 for others)
  pub fn get_neighbors(&self, x: usize, y:usize) -> Vec<(usize, usize)> {
    get_neighbors(
      self.board[x][y].delta_neighbors(),
      x,
      y,
      self.width,
      self.height
    )
  }
}


/// Returns (x+dx % width, y+dy%height), plus all the edge cases/conversions
/// If `wraps`, always returns a value.
/// Else, x+dx < 0 or x+dx >= width returns None, and likewise with y/height.
pub fn delta_to_neighbor(
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

/// Get a list of absolute coordinates of neighbors
/// Given deltas, the pixel they apply to, and the board width/height 
/// Conveneince wrapping over delta_to_neighbor.
pub fn get_neighbors(
  deltas: Vec<(isize, isize)>, x: usize, y: usize, width: usize, height: usize
) -> Vec<(usize, usize)> {
  // Given resel class + x,y + width,height, get the neighborhood of (x,y) coordinates

  deltas
    .into_iter()
    .filter_map(|(dx, dy)| delta_to_neighbor(x, y, dx, dy, width, height, true))
    .collect()
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
  fn can_instantiate_reselboard_test() {
    for filename in [
      "./src/testing/test_01_new-palette.png",
      "./src/testing/test_02_new-palette_1.png",
      "./src/testing/test_02_new-palette.png",
      "./src/testing/test_03_01.png",
      "./src/testing/test_03_02.png",
      "./src/testing/test_03_03.png",
      "./src/testing/test_03_04.png",
      "./src/testing/test_03_alloff.png",
      "./src/testing/test_03_allon.png",
      "./src/testing/test_04.png",
      "./src/testing/test_05_01.png",
      "./src/testing/test_05_02.png",
      "./src/testing/test_05_03.png",
      "./src/testing/test_05_04.png",
      "./src/testing/test_05_05.png",
      "./src/testing/test_05_06.png",
      "./src/testing/test_06.png",
    ] {
      let image = load_image_from_filename(filename).unwrap();
      let reselboard = image_to_reselboard(image.clone());
      assert_eq!(
        reselboard.board, image_to_vecvecresel(&image)
      );
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

      // Cases added during debug
      (0, 0, 0, -1, 3, 5, true, Some((0, 4))),

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
      // Specific tests to debug issues
      (Resel::AND, 0, 0, 3, 5, vec![(0,1), (1,0), (2,0), (0,4)]),
      (Resel::AND, 2, 4, 3, 5, vec![(0,4), (2,0), (1,4), (2,3)]),
      (
        Resel::WireOrangeOn, 0, 0, 3, 5,
        vec![(1,0), (1,1), (0,1), (2,1), (2,0), (2,4), (0,4), (1,4)]
      ),
      (
        Resel::WireOrangeOn, 2, 4, 3, 5,
        vec![(0,4), (0,0), (2,0), (1,0), (1,4), (1,3), (2,3), (0,3)]
      ),
    ] {
      let neighbors_1: HashSet<(usize, usize)> = get_neighbors(
        resel.delta_neighbors(), x, y, width, height
      ).into_iter().collect();

      let neighbors_2: HashSet<(usize, usize)> = neighbors.into_iter().collect();
      assert_eq!(neighbors_1, neighbors_2);

    }
  }
}

// eof