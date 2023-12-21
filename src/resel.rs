//! resel.rs: Conversions between Resels, Rgba pixels, and chars.
//! 
//! A "resel" is a "reso pixel".
//! 
//! An image or text file is converted to a Vec<Vec<Resel>> as the first step
//! of preprocessing. Regions of resels in the Vec<Vec<Resel>> form the
//! logical nodes which form Reso circuits.
//! 
//! A Resel can be converted to/from:
//! - A pixel defined as image::Rgba
//! - An RGB pixel defined as (u8,u8,u8)
//! - A character
//! 
//! The palette is defined as follows:
//!
//! | Color          | Meaning               | Hex code       | RGB               | ASCII |
//! | -------------- | --------------------- | ---            | ----------------- | ----- |
//! | Dark  orange   | Orange wire (off)     | ```#804000```  | `(128,  64,   0)` | `o`   |
//! | Bright orange  | Orange wire (on)      | ```#ff8000```  | `(255, 128,   0)` | `O`   |
//! | Dark sapphire  | Sapphire wire (off)   | ```#004080```  | `(  0,  64, 128)` | `s`   |
//! | Bright sapphire| Sapphire wire (on)    | ```#0080ff```  | `(  0, 128, 255)` | `S`   |
//! | Dark lime      | Lime wire (off)       | ```#408000```  | `(64,  128,   0)` | `l`   |
//! | Bright lime    | Lime wire (on)        | ```#80ff00```  | `(128, 255,   0)` | `L`   |
//! | Dark teal      | AND logic node        | ```#008040```  | `(  0, 128,  64)` | `&`   |
//! | Bright teal    | XOR logic node        | ```#00ff80```  | `(  0, 255, 128)` | `^`   |
//! | Dark purple    | Input (wire to node)  | ```#400080```  | `( 64,   0, 128)` | `+`   |
//! | Bright purple  | Output (node to wire) | ```#8000ff```  | `(128,   0, 255)` | `=`   |
//! 
//! Example:
//! 
//! ```rust
//! // Convert to a Resel
//! let (r, g, b) = (  0, 255, 128);
//! let rgba = Rgba([r, g, b, 255]);
//! let resel_from_rgb_tuple = Resel::from((r,g,b));
//! let resel_from_rgba      = Resel::from(rgba);
//! let resel_from_char      = Resel::from("^");
//! let resel                = Resel::XOR;
//! 
//! assert_eq!(resel, resel_from_rgb_tuple);
//! assert_eq!(resel, resel_from_rgba);
//! assert_eq!(resel, resel_from_char);
//! 
//! // Convert from a Resel
//! assert_eq!((r,g,b), <(u8,u8,u8)>::from(resel));
//! assert_eq!(rgba,    <Rgba<u8>>::from(resel));
//! assert_eq!("^",     <&str>::from(resel));
//! 
//! ```

/*
TODOs:
- Consider using `char` rather than `str`. 
*/

use image::{Rgba};


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

// Palettes must be kept in the same order as one another

/// Palette of Resels for easy iteration
pub const PALETTE_RESEL: [Resel; 11] = [
  Resel::WireOrangeOff,
  Resel::WireOrangeOn,
  Resel::WireSapphireOff,
  Resel::WireSapphireOn,
  Resel::WireLimeOff,
  Resel::WireLimeOn,
  Resel::AND,
  Resel::XOR, 
  Resel::Input,
  Resel::Output,
  Resel::Empty
];

/// Palette of rgb u8u8u8 Resel values for easy iteration
pub const PALETTE_U8U8U8: [(u8,u8,u8); 11] = [
  (128,  64,   0), // WireOrangeOff
  (255, 128,   0), // WireOrangeOn
  (  0,  64, 128), // WireSapphireOff
  (  0, 128, 255), // WireSapphireOn
  (64,  128,   0), // WireLimeOff
  (128, 255,   0), // WireLimeOn
  (  0, 128,  64), // AND
  (  0, 255, 128), // XOR
  ( 64,   0, 128), // Input
  (128,   0, 255), // Output
  (0,     0,   0), // Empty
];

/// Palette of image::Rgba Resel values for easy iteration
pub const PALETTE_RGBA: [Rgba<u8>; 11] = [
  Rgba([128,  64,   0, 255]), // WireOrangeOff
  Rgba([255, 128,   0, 255]), // WireOrangeOn
  Rgba([  0,  64, 128, 255]), // WireSapphireOff
  Rgba([  0, 128, 255, 255]), // WireSapphireOn
  Rgba([64,  128,   0, 255]), // WireLimeOff
  Rgba([128, 255,   0, 255]), // WireLimeOn
  Rgba([  0, 128,  64, 255]), // AND
  Rgba([  0, 255, 128, 255]), // XOR
  Rgba([ 64,   0, 128, 255]), // Input
  Rgba([128,   0, 255, 255]), // Output
  Rgba([0,     0,   0, 255]), // Empty
];

/// Palette of str Resel values for easy iteration
pub const PALETTE_STR: [&str; 11] = [
  "o", // WireOrangeOff
  "O", // WireOrangeOn
  "s", // WireSapphireOff
  "S", // WireSapphireOn
  "l", // WireLimeOff
  "L", // WireLimeOn
  "&", // AND
  "^", // XOR
  "+", // Input
  "=", // Output
  " ", // Empty
];

/// Converts rgb <(u8,, u8, u8)> pixel to Resel
impl From<(u8, u8, u8)> for Resel {
  fn from(rgb: (u8, u8, u8)) -> Self {
    match rgb {
      (128,  64,   0) => Resel::WireOrangeOff,
      (255, 128,   0) => Resel::WireOrangeOn,
      (  0,  64, 128) => Resel::WireSapphireOff,
      (  0, 128, 255) => Resel::WireSapphireOn,
      (64,  128,   0) => Resel::WireLimeOff,
      (128, 255,   0) => Resel::WireLimeOn,
      (  0, 128,  64) => Resel::AND,
      (  0, 255, 128) => Resel::XOR,
      ( 64,   0, 128) => Resel::Input,
      (128,   0, 255) => Resel::Output,
      _               => Resel::Empty,
    }
  }
}

/// Converts image::Rgba to Resel
impl From<Rgba<u8>> for Resel {
  fn from(rgba: Rgba<u8>) -> Self {
    match rgba {
      Rgba([128,  64,   0, 255]) => Resel::WireOrangeOff,
      Rgba([255, 128,   0, 255]) => Resel::WireOrangeOn,
      Rgba([  0,  64, 128, 255]) => Resel::WireSapphireOff,
      Rgba([  0, 128, 255, 255]) => Resel::WireSapphireOn,
      Rgba([64,  128,   0, 255]) => Resel::WireLimeOff,
      Rgba([128, 255,   0, 255]) => Resel::WireLimeOn,
      Rgba([  0, 128,  64, 255]) => Resel::AND,
      Rgba([  0, 255, 128, 255]) => Resel::XOR,
      Rgba([ 64,   0, 128, 255]) => Resel::Input,
      Rgba([128,   0, 255, 255]) => Resel::Output,
      _ => Resel::Empty,
    }
  }
}

/// Converts &str to Resel
impl From<&str> for Resel {
  fn from(c: &str) -> Self {
    match c {
      "o" => Resel::WireOrangeOff  ,
      "O" => Resel::WireOrangeOn   ,
      "s" => Resel::WireSapphireOff,
      "S" => Resel::WireSapphireOn ,
      "l" => Resel::WireLimeOff    ,
      "L" => Resel::WireLimeOn     ,
      "&" => Resel::AND            ,
      "^" => Resel::XOR            ,
      "+" => Resel::Input          ,
      "=" => Resel::Output         ,
       _  => Resel::Empty          ,
    }
  }
}

/// Converts Resel to rgb <(u8, u8, u8)>
impl From<Resel> for (u8, u8, u8) {
  fn from(resel: Resel) -> Self {
    match resel {
      Resel::WireOrangeOff   => (128,  64,   0),
      Resel::WireOrangeOn    => (255, 128,   0),
      Resel::WireSapphireOff => (  0,  64, 128),
      Resel::WireSapphireOn  => (  0, 128, 255),
      Resel::WireLimeOff     => (64,  128,   0),
      Resel::WireLimeOn      => (128, 255,   0),
      Resel::AND             => (  0, 128,  64),
      Resel::XOR             => (  0, 255, 128),
      Resel::Input           => ( 64,   0, 128),
      Resel::Output          => (128,   0, 255),
      Resel::Empty           => (0,     0,   0)
    }
  }
}
/// Converts Resel to image::Rgba
impl From<Resel> for Rgba<u8> {
  fn from(resel: Resel) -> Self {
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
      Resel::Empty           => Rgba([0,     0,   0, 255])
    }
  }
}
/// Converts Resel to str
impl From<Resel> for &str {
  fn from(resel: Resel) -> Self {
    match resel {
      Resel::WireOrangeOff   => "o",
      Resel::WireOrangeOn    => "O",
      Resel::WireSapphireOff => "s",
      Resel::WireSapphireOn  => "S",
      Resel::WireLimeOff     => "l",
      Resel::WireLimeOn      => "L",
      Resel::AND             => "&",
      Resel::XOR             => "^",
      Resel::Input           => "+",
      Resel::Output          => "=",
      Resel::Empty           => " ",
    }
  }
}


impl Resel {

  /// Check if one Resel is the same as the other.
  /// Useful because WireOrangeOn is the "same" as WireOrangeOff
  pub fn same(self, other: Resel) -> bool {
    match (self, other) {
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
        // All other cases: Match true if resels are equal
        self == other
      }
    }
  }


  pub fn is_wire(&self) -> bool {
    match self {
      Resel::WireOrangeOff   | Resel::WireOrangeOn   |
      Resel::WireSapphireOff | Resel::WireSapphireOn |
      Resel::WireLimeOff     | Resel::WireLimeOn
        => true,
      _ => false
    }
  }

  pub fn is_logic(&self) -> bool {
    match self {
      Resel::AND | Resel::XOR => true,
      _ => false
    }
  }

  pub fn wire_state(&self) -> Option<bool> {
    match self {
      Resel::WireOrangeOn
      | Resel::WireSapphireOn
      | Resel::WireLimeOn
      => Some(true),
      Resel::WireOrangeOff
      | Resel::WireSapphireOff
      | Resel::WireLimeOff
      => Some(false),
      _ => None
    }
  }

  pub fn is_empty(&self)  -> bool { *self == Resel::Empty }
  pub fn is_input(&self)  -> bool { *self == Resel::Input }
  pub fn is_output(&self) -> bool { *self == Resel::Output }
  pub fn is_io(&self)     -> bool { self.is_input() || self.is_output() }


  /// resel.delta_neighbors() returns the relative (x,y) of neighboring cells.
  /// Used in the region mapper to find contiguous regions.
  /// 
  /// Wire neighborhoods are orthogonal and diagonal, meaning wire resels
  /// touching on the sides or corners are part of the same region.
  /// All others are only orthogonally contiguous. 
  pub fn delta_neighbors(&self) -> Vec<(isize, isize)> {
    match self {
      Resel::WireOrangeOff   | Resel::WireOrangeOn   |
      Resel::WireSapphireOff | Resel::WireSapphireOn |
      Resel::WireLimeOff     | Resel::WireLimeOn
        => vec![(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1)],
      Resel::Empty => Vec::new(),
      _ => vec![(1,0),(0,1),(-1,0),(0,-1)],
    }
  }
}

#[cfg(test)]
mod resel_conversion_tests {
  use super::*;

  #[test]
  fn test_palettes() {
    // this test probably makes a lot of the below redundant

    // assert each same length
    assert_eq!(
      PALETTE_RESEL.len(), PALETTE_U8U8U8.len()
    );
    assert_eq!(
      PALETTE_RESEL.len(), PALETTE_RGBA.len()
    );
    assert_eq!(
      PALETTE_RESEL.len(), PALETTE_STR.len()
    );

    // assert equal
    for idx in 1..PALETTE_RESEL.len() {
      let resel  = PALETTE_RESEL[idx];
      let u8u8u8 = PALETTE_U8U8U8[idx];
      let rgba   = PALETTE_RGBA[idx];
      let cc     = PALETTE_STR[idx];

      // test resel == resel::from(x)
      assert_eq!(
        resel,
        Resel::from(u8u8u8)
      );
      assert_eq!(
        resel,
        Resel::from(rgba)
      );
      assert_eq!(
        resel,
        Resel::from(cc)
      );

      // test x == resel.into()
      assert_eq!(
        u8u8u8,
        <(u8,u8,u8)>::from(resel)
      );
      assert_eq!(
        rgba,
        <Rgba<u8>>::from(resel)
      );
      assert_eq!(
        cc,
        <&str>::from(resel)
      );
    }
  }

  #[test]
  fn test_convert_from_u8u8u8() {
    for (r, g, b) in PALETTE_U8U8U8 {
      // Instantiate Rgba, resel from (r,g,b,a)
      let rgba = Rgba([r, g, b, 255]);
      let resel_from_raw_rgb = Resel::from((r, g, b));
      let resel_from_rgba    = Resel::from(rgba);
      print!(".");

      // ... And check it converts back correctly
      assert_eq!(
        (r, g, b),
        resel_from_raw_rgb.into()
      );
      assert_eq!(
        (r, g, b),
        resel_from_rgba.into()
      );
    }
  }
  #[test]
  fn test_convert_from_resel() {
    for resel in [
      Resel::WireOrangeOff,
      Resel::WireOrangeOn,
      Resel::WireSapphireOff,
      Resel::WireSapphireOn,
      Resel::WireLimeOff,
      Resel::WireLimeOn,
      Resel::AND,
      Resel::XOR,
      Resel::Input,
      Resel::Output
    ] {
      // Convert Resel to (u8,u8,u8), Rgba, and char and back
      let (r, g, b): (u8, u8, u8) = resel.into();
      let rgba:      Rgba<u8>     = resel.into();
      let cc:        &str         = resel.into();

      assert_eq!(
        Resel::from((r, g, b)),
        resel
      );
      assert_eq!(
        Resel::from(rgba),
        resel
      );
      assert_eq!(
        Resel::from(cc),
        resel
      );
    }
  }
  
  #[test]
  fn test_same() {
    for (resel1, resel2) in [
      (Resel::WireOrangeOff, Resel::WireOrangeOn),
      (Resel::WireSapphireOff, Resel::WireSapphireOn),
      (Resel::WireLimeOff, Resel::WireLimeOn)
    ] {
      assert!(resel1.same(resel1));
      assert!(resel1.same(resel2));
      assert!(resel2.same(resel1));
      assert!(resel2.same(resel2));
    }

    for resel in [
      Resel::WireOrangeOff,
      Resel::WireOrangeOn,
      Resel::WireSapphireOff,
      Resel::WireSapphireOn,
      Resel::WireLimeOff,
      Resel::WireLimeOn,
      Resel::AND,
      Resel::XOR,
      Resel::Input,
      Resel::Output
    ] {
      assert!(resel.same(resel));
    }
  }
}

// eof