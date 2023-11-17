use std::collections::HashMap;
use image::{GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage};

/* TODO:
- Impl idiomatic (RGB, &str)--Resel conversion
- Tests
- Docstrings
*/

/// Enum of all the classes a resel can have
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resel {
  WireOrangeOff,
  WireOrangeOn,
  WireSapphireOff,
  WireSapphireOn,
  WireLimeOff,
  WireLimeOn,
  AND,
  XOR, 
  Input,
  Output,
  Empty
}

/// Mapping of of (R, G, B, A)  to Resel class.
pub fn rgbas_to_resel(r: u8, g: u8, b: u8, a: u8) -> Resel {
  match (r, g, b, a) {
    (128,  64,   0, 255) => Resel::WireOrangeOff,
    (255, 128,   0, 255) => Resel::WireOrangeOn,
    (  0,  64, 128, 255) => Resel::WireSapphireOff,
    (  0, 128, 255, 255) => Resel::WireSapphireOn,
    (64,  128,   0, 255) => Resel::WireLimeOff,
    (128, 255,   0, 255) => Resel::WireLimeOn,
    (  0, 128,  64, 255) => Resel::AND,
    (  0, 255, 128, 255) => Resel::XOR,
    ( 64,   0, 128, 255) => Resel::Input,
    (128,   0, 255, 255) => Resel::Output,
    _ => Resel::Empty,
  }
}

/// Mapping of image::Rgba<u8> to Resel class. Convenience.
pub fn rgba_to_resel(pixel: Rgba<u8>) -> Resel {
  rgbas_to_resel(pixel[0], pixel[1], pixel[2], pixel[3])
}

/// Mapping of Resel class to RGBA value.
pub fn resel_to_rgba(resel: Resel) -> Rgba<u8> {
  match resel {
    Resel::WireOrangeOff   => Rgba([128,  64,   0, 255]),
    Resel::WireOrangeOn    => Rgba([255, 128,   0, 255]),
    Resel::WireSapphireOff => Rgba([  0,  64, 128, 255]),
    Resel::WireSapphireOn  => Rgba([  0, 128, 255, 255]),
    Resel::WireLimeOff     => Rgba([64,  128,   0, 255]),
    Resel::WireLimeOn      => Rgba([128, 255,   0, 255]),
    Resel::AND             => Rgba([  0, 128,  64, 255]),
    Resel::XOR             => Rgba([  0, 255, 128, 255]),
    Resel::Input           => Rgba([ 64,   0, 128, 255]),
    Resel::Output          => Rgba([128,   0, 255, 255]),
    Resel::Empty           => Rgba([0, 0, 0, 0])
  }
}

// Map an rgba image to a 2D grid of Resels.
pub fn image_to_reselboard(img: &DynamicImage) -> Vec<Vec<Resel>> {
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

/// Unusued; for text-based reselboard.
pub fn ascii_to_resel(c: char) -> Resel {
  match c {
    'o' => Resel::WireOrangeOff,
    'O' => Resel::WireOrangeOn,
    's' => Resel::WireSapphireOff,
    'S' => Resel::WireSapphireOn,
    'l' => Resel::WireLimeOff,
    'L' => Resel::WireLimeOn,
    '&' => Resel::AND,
    '^' => Resel::XOR,
    '+' => Resel::Input,
    '=' => Resel::Output,
    ' ' => Resel::Empty,
    _ => Resel::Empty,
  }
}

/// Unusued; for text-based reselboard.
pub fn resel_to_ascii(resel: Resel) -> char {
  match resel {
    Resel::WireOrangeOff   => 'o',
    Resel::WireOrangeOn    => 'O',
    Resel::WireSapphireOff => 's',
    Resel::WireSapphireOn  => 'S',
    Resel::WireLimeOff     => 'l',
    Resel::WireLimeOn      => 'L',
    Resel::AND             => '&',
    Resel::XOR             => '^',
    Resel::Input           => '+',
    Resel::Output          => '=',
    Resel::Empty           => ' ',
  }
}

pub fn is_resel_same_class(resel1: Resel, resel2: Resel) -> bool {
  match (resel1, resel2) {
    // Handle wire special cases
    // there has to be a better way to do this
    ( Resel::WireOrangeOff | Resel::WireOrangeOn,
      Resel::WireOrangeOn  | Resel::WireOrangeOff
    ) | (
      Resel::WireSapphireOff | Resel::WireSapphireOn,
      Resel::WireSapphireOn  | Resel::WireSapphireOff
    ) | (
      Resel::WireLimeOff | Resel::WireLimeOn,
      Resel::WireLimeOn | Resel::WireLimeOff
    ) => { true },
    (_, _) => { 
      // All other cases: Match true if resels are the same class
      resel1 == resel2
    }
  }
}