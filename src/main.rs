/// main.rs: Reso CLI
/// 
/// 


mod resel;
#[allow(dead_code)]
mod reselboard;
#[allow(unused_parens)]
mod regionmap;
mod incidencemap;
#[allow(unused_parens)]
mod resocircuit;

use std::collections::HashMap;
use image::{
  GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage
};


use resel::{Resel
};
use reselboard::{
  image_to_vecvecresel,
  load_image_from_filename,
};
use regionmap::{RegionMap};
use resocircuit::{ResoCircuit};


pub fn main() {
  println!("Hello joyous world!");
  println!("Goodbye cruel world!");
}