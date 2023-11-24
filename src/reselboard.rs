/* reselboard.rs

TODOs:
- Load from .txt, image filename
- Tests!
- Then region mapping 
*/

use crate::resel::{Resel};
use image::{DynamicImage, GenericImageView};


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



#[cfg(test)]
mod load_image_tests {
  use super::*;
  #[test]
  fn load_image_doesnt_exist() {
    assert!(load_image_from_filename("this_does_not_exist.png").is_none())
    // assert None
  }

  #[test]
  fn load_image_does_exist() {
    assert!(load_image_from_filename("./src/testing/test_01.png").is_some())
  }

  #[test]
  fn load_image_test() {
    // Test pixels on loaded test images
  }
}

#[cfg(test)]
mod reselboard_conversion_tests {
}


/*
ReselBoard: Vec<Vec<Resel>>

Also:
- Stores regions 

*/