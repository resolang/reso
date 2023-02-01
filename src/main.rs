use image::{GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage};

/* Reso concepts
 * 
 * An image is a 2D array of pixels, each pixel has an RGBA color.
 * 
 * Meanwhile, a "reso circuit": 
 *  - Has a reso-board, which is an ND array of resels, each resel has a resel class.
 *  - Each resel belongs to a region.
 *  - The graph of regions makes the circuit.
 */

// image loading
fn load_image_from_filename(filename: &str) -> DynamicImage {
    // Load the image from the file (copilot)
    let img = image::open(filename).expect("File not found, sorry!");
    let (width, height) = img.dimensions();
    println!("Loaded {} ({}x{} px).", &filename, width, height);
    img
}

// resel conversion code
// enum of resel classes
#[derive(Debug, Clone, Copy, PartialEq)]
enum Resel {
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

// Map pixel color to resel class
fn rgba_to_resel(r: u8, g: u8, b: u8, a: u8) -> Resel {
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

// Map of resel class to pixel color
fn resel_to_rgba(resel: Resel) -> Rgba<u8> {
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
// (img: &DynamicImage means `img` is a *reference* to a `DynamicImage)
fn image_to_reselboard(img: &DynamicImage) -> Vec<Vec<Resel>> {
    let (width, height) = img.dimensions();
    let mut reselboard = vec![vec![Resel::Empty; height as usize]; width as usize];
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let resel = rgba_to_resel(pixel[0], pixel[1], pixel[2], pixel[3]);
            reselboard[x as usize][y as usize] = resel;
        }
    }
    reselboard
    // TODO! This can be a fixed size array, I think?
    // see https://stackoverflow.com/questions/59164456/
}

// compile to circuit elements

//todo: ascii_to_resel, resel_to_ascii, resoascii_to_resoboard, resoboard_to_resoascii

//todo: compile_resoboard

/* todo: struct ResoCircuit
    // aux drawing data
    - pixels_by_region, region_by_pixel

    // compiled graph
    - class_by_region
    - wire_nodes, input_nodes, logic_nodes, output_nodes
    - wires_to_inputs, inputs_to_logic, inputs_to_outputs, outputs_to_wires
        ... sparse adjacency via hashmap? (custom hashmap region idx -> vec<region idx>?) we expect sparse indexing ints to ints
        ... direct adjacency matrix? (num regions x num regions)?
        ... sparse adjacency via vec? vec<vec<usize>>, mostly empty. e.g. [[], [], [], [3, 4], []] etc
        ... Just use HashMap, probably, then revisit when I know more

    // temp vars which change while running
    - wire_state
    - input_state
    - output_state
*/

// todo: rename pixel_ to resel_. 
// pixel_region_mapping_from_reselboard(reselboard, width, height) -> region_by_pixel, pixels_by_region
fn pixel_region_mapping_from_reselboard(
    reselboard: &Vec<Vec<Resel>>,
    width: usize,
    height: usize,
) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {
    let mut region_idx: usize = 0;
    let mut visited:     Vec<Vec<bool>> = vec![vec![false; height as usize]; width as usize];
    // todo: visited is redundant, just check region_by_pixel? defaults to 9
    let mut region_by_pixel: Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
    let mut pixels_by_region: Vec<Vec<(usize, usize)>> = vec![Vec::new()];

    // pixels_by_region[0] empty-- we index regions starting with 1.
    // 'region_by_pixel[x][y] = 0' means [x][y] doesn't have a region assignment
    pixels_by_region.push(Vec::new());

    // todo: This can run class-by-region as well.
    // (... And all the elements in the compilation step!)
    // Wire nodes: Set as off by default, to "on" if we see one

    // For each pixel
    for x in 0..width {
        for y in 0..height {
            if visited[x][y] {
                // Ignore it if already visited!
            } else {
                // Pixel is not visited -- this pixel marks a new region!
                // First, update our region count, and prepare a new list of region pixels to populate
                // (region_idx 0 is skipped intentionally, and pixels_by_region[0] should stay empty.)
                // (on the first loop, region_idx == 1, and length of pixels_by_region.len() == 2)
                region_idx += 1;
                pixels_by_region.push(Vec::new());

                // Now, let's explore this pixel, and all of our neighbors, one-by-one.
                // (The starting pixel counts as the first neighbor.)
                let mut neighbors: Vec<(usize, usize)> = Vec::new();
                neighbors.push((x, y));

                // Explore neighboring contiguous pixels.
                // If a neighbor pixel is contiguous, add it to `neighbors`
                while !neighbors.is_empty() {
                    let (x, y) = neighbors.pop().unwrap();

                    // Record info about the newly-inducted pixel to our region!
                    region_by_pixel[x][y] = region_idx; // Mark this region on our map
                    visited[x][y] = true; // Mark this pixel as visited
                    // TODO
                    // (... what did I have set 'todo' here?) fix the clone?
                    pixels_by_region[region_idx].push((x.clone(),y.clone())); // Remember this pixel belongs to this region

                    // Check contiguity.
                    for (dx, dy) in {
                        // (dx,dy) = possible directions for adjacent pixels.
                        // wires can be contiguous orthogonally and diagonally,
                        // every other class can only be contiguous orthogonally.
                        if [
                            Resel::WireOrangeOff, Resel::WireOrangeOn,
                            Resel::WireSapphireOff, Resel::WireSapphireOn,
                            Resel::WireLimeOff, Resel::WireLimeOn
                        ].contains(&reselboard[x][y]) {
                            // Diagonal neighbors
                            [(1,0), (1, height), (0, height), (width, height), (width, 0), (width, 1), (0, 1), (1,1)]
                        } else if [
                            Resel::AND, Resel::XOR, Resel::Input, Resel::Output
                        ].contains(&reselboard[x][y]) {
                            // Ortho neighbors   --  cheap hack, pad with (0,0)s.
                            [(1,0), (0,height), (width, 0), (0, 1), (0,0), (0,0), (0,0), (0,0)]
                        } else {
                            // No neighbors. Should not be possible
                            [(0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)]
                        }
                    }.iter() { // for (dx, dy) in ..neighbors to check.. {
                        match (
                            &reselboard[x][y], // Our pixel
                            &reselboard[(x + dx) % width][(y + dy)%height] // Neighbor pixel
                        ) {
                            // Simple case where the resels match, add the pixel to our bag of neighbors
                            (resel_a, resel_b) if (resel_a == resel_b && dx != dy) => neighbors.push(
                                ((x + dx) % width, (y + dy)%height)
                            ),
                            // Wires match, but have different on/off values, add the pixel to our bag of neighbors
                            (
                                // e.g. current resel and adjacent resel are orange
                                Resel::WireOrangeOff | Resel::WireOrangeOn,
                                Resel::WireOrangeOff | Resel::WireOrangeOn
                            ) | (
                                // e.g. current resel and adjacent resel are sapphire
                                Resel::WireSapphireOff | Resel::WireSapphireOn,
                                Resel::WireSapphireOff | Resel::WireSapphireOn
                            ) | (
                                // e.g. current resel and adjacent resel are lime
                                Resel::WireLimeOff | Resel::WireLimeOn,
                                Resel::WireLimeOff | Resel::WireLimeOn
                            ) if dx != dy => neighbors.push(
                                ((x + dx) % width, (y + dy)%height)
                            ),
                            // Else, do nothing
                            (_, _) => ()
                            // note: setting region_by_pixel and pixel_by_region
                            // happens at the start of this while loop
                      } // match expression to check contiguity;
                        // if contiguous, add to 'neighbors'
                    } // loop which checks adjacent resels for contiguity.
                } // while loop which iterates over all neighbors in a region,
                  // updating the region_by_pixel and pixels_by_region mappings.
            } // consider resel.
              // if in visited, ignore, look at the next resel.
              // else, this is a new region, let's look for all adjacent pixels
        } // for each y
    } // for each x

    (region_by_pixel, pixels_by_region)
}

fn main() {
    let img = load_image_from_filename("test.png");
    let (width, height) = img.dimensions();
    println!("{}x{}", width, height);
    println!("{:?}", image_to_reselboard(&img));
    //println!("{:?}", pixel_region_mapping_from_reselboard(&image_to_reselboard(&img), width as usize, height as usize));
}