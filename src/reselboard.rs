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
