/* reselboard.rs

TODOs:
- Load from .txt, image filename
- Tests
- Then region mapping
*/

mod resel;
use resel::Resel;
use image::{Rgba, DynamicImage};

struct ReselBoard {
  board: Vec<Vec<Resel>>,
  image: &DynamicImage,
}

// Instantiate Vec<Vec<Resel>> 
impl From<&DynamicImage> for Vec<Vec<Resel>> {
  fn from(img: &DynamicImage) -> Vec<Vec<Resel>> {
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