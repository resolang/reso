
use std::collections::HashMap;
use image::{
  GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage
};

mod resel;
mod reselboard;
mod regionmap;
mod incidencemap;

use resel::{Resel
};

use reselboard::{
  image_to_vecvecresel,
  load_image_from_filename,
};
use regionmap::{
  region_map_from_reselboard,
  RegionMap
};



/*
todo:
- rename "region" to "node"? be consistent
- Clean up code, comments
- Clean up docs
  - architecture
  - algorithms
  - each file
  - readme.md
- Code:
  - ResoCircuit.rs
  - main.rs CLI
- Far later:
  - See `todo redundant` when I'm better at Rust.
    - How to restructure?
    - e.g. instead of `wire_regions[wi]` maybe `dense[Class::Wire][wi]`?
  - resolang.dev site
  - IDE / tools?
  - Debugger which shows ReselBoard, RegionMap, AdjacencyMap, ResoCircuit state
*/

/*
ResoCircuit ideas
- Should hold a RegionMap, IncidenceMap at least?
  - Some can be Optional to save memory
- Needs to hold temporary state for wires, inputs, logics, and outputs
- Optional render step
- Simple iteration
- Serialize to format

#[derive(Debug, Clone)]
struct ResoCircuit {
}

*/