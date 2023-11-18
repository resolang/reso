/*
todo
impl From<&DynamicImage> for Resel {
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
*/