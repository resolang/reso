
use std::collections::HashMap;
use image::{
  GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage
};

mod resel;
mod reselboard;
mod regionmap;
mod incidencemap;
mod resocircuit;

use resel::{Resel
};

use reselboard::{
  image_to_vecvecresel,
  load_image_from_filename,
};
use regionmap::{RegionMap};
use resocircuit::{ResoCircuit};
