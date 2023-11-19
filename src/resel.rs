/*
resel.rs

A "resel" is a "reso pixel". A resel can be converted to/from an (R,G,B) or a character
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

// Convert (u8,u8,u8), Rgba<u8>, and &str char to Resel
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
      _ => Resel::Empty,
    }
  }
}

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

// Convert Resel to (u8,u8,u8), Rgba<u8>, and &str char
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
  pub fn same(self, other: Resel) -> bool {
    // Used when checking adjacent regions.
    // (Adjacent wires, e.g. OOOooo, compile to the same wire OOOOOO)
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
  /*
  // unused
  pub fn is_wire(&self) -> bool {
    match self {
      Resel::WireOrangeOff | Resel::WireOrangeOn |
      Resel::WireSapphireOff | Resel::WireSapphireOn |
      Resel::WireLimeOff | Resel::WireLimeOn => true,
      _ => false
    }
  }

  pub fn is_logic(&self) -> bool {
    match self {
      Resel::AND | Resel::XOR => true,
      _ => false
    }
  }

  pub fn is_io(&self) -> bool {
    match self {
      Resel::Input | Resel::Output => true,
      _ => false
    }
  }
  */
}



#[cfg(test)]
mod resel_conversion_tests {
  use super::*;

  #[test]
  fn test_convert_from_rgba() {
    for (r, g, b) in [
      (128,  64,   0),
      (255, 128,   0),
      (  0,  64, 128),
      (  0, 128, 255),
      (64,  128,   0),
      (128, 255,   0),
      (  0, 128,  64),
      (  0, 255, 128),
      ( 64,   0, 128),
      (128,   0, 255),
    ] {
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

/*
#[cfg(test)]
mod more_tests {
  #[test]
}
*/