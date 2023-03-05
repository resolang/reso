use std::collections::HashMap;
use image::{GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage};


/// Return image::DynamicImage given filename
fn load_image_from_filename(filename: &str) -> DynamicImage {
    // todo -- should be String and not &str?
    // Load the image from the file (copilot)
    let img = image::open(filename).expect("File not found, sorry!");
    let (width, height) = img.dimensions();
    println!("Loaded {} ({}x{} px).", &filename, width, height);
    img
}

/// Enum of all the classes a resel can have
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

/// Mapping of of (R, G, B, A)  to Resel class.
fn rgbas_to_resel(r: u8, g: u8, b: u8, a: u8) -> Resel {
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

/// Mapping of image::Rgba<u8> to Resel class.
fn rgba_to_resel(pixel: Rgba<u8>) -> Resel {
    rgbas_to_resel(pixel[0], pixel[1], pixel[2], pixel[3])
}

/// Mapping of Resel class to RGBA value.
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
fn image_to_reselboard(img: &DynamicImage) -> Vec<Vec<Resel>> {
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
    // TODO! This can be a fixed size array, I think?
    // see https://stackoverflow.com/questions/59164456/
}

/// Unusued; for text-based reselboard.
fn ascii_to_resel(c: char) -> Resel {
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
fn resel_to_ascii(resel: Resel) -> char {
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


/// Given a reselboard, find and index regions of adjacent elements.
fn resel_region_mapping_from_reselboard(
    reselboard: &Vec<Vec<Resel>>,
    width: usize,
    height: usize,
) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {
    let mut region_idx: usize = 0;
    let mut visited:     Vec<Vec<bool>> = vec![vec![false; height as usize]; width as usize];
    // todo: visited is redundant, just check region_by_resel? defaults to 0
    let mut region_by_resel: Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
    let mut resels_by_region: Vec<Vec<(usize, usize)>> = vec![Vec::new()];

    // resels_by_region[0] empty-- we index regions starting with 1.
    // 'region_by_resel[x][y] = 0' means [x][y] doesn't have a region assignment
    resels_by_region.push(Vec::new());

    // todo: This can run class-by-region as well.
    // (... And all the elements in the compilation step!)
    // Wire nodes: Set as off by default, to "on" if we see one

    // For each resel
    for x in 0..width {
        for y in 0..height {
            if visited[x][y] {
                // Ignore it if already visited!
            } else if reselboard[x][y] == Resel::Empty {
                // Mark as visited, but don't count it as a region, since it is Empty
                // (i.e. not a Resel)
                visited[x][y] = true
            } else {
                // Resel is not visited -- this resel marks a new region!
                // First, update our region count, and prepare a new list of region resel to populate
                // (region_idx 0 is skipped intentionally, and resels_by_region[0] should stay empty.)
                // (on the first loop, region_idx == 1, and length of resels_by_region.len() == 2)
                region_idx += 1;
                resels_by_region.push(Vec::new());

                // Now, let's explore this resel, and all of our neighbors, one-by-one.
                // (The starting resel counts as the first neighbor.)
                let mut neighbors: Vec<(usize, usize)> = Vec::new();
                
                neighbors.push((x, y));

                // Explore neighboring contiguous resels.
                // If a neighbor resel is contiguous, add it to `neighbors`
                while !neighbors.is_empty() {
                    let (x, y) = neighbors.pop().unwrap();

                    //println!("({:?}, {:?}), idx {:?} ========", x, y, region_idx);

                    // Record info about the newly-inducted resel to our region!
                    region_by_resel[x][y] = region_idx; // Mark this region on our map
                    visited[x][y] = true; // Mark this resel as visited
                    // TODO
                    // (... what did I have set 'todo' here?) refactor to remove the clone?
                    // this is an ugly nest of logic already...
                    resels_by_region[region_idx].push((x.clone(),y.clone()));
                    // Remember this resel belongs to this region

                    // Check contiguity.
                    for (dx, dy) in {
                        // (dx,dy) = possible directions for adjacent resels.
                        // wires can be contiguous orthogonally and diagonally,
                        // every other class can only be contiguous orthogonally.
                        if [
                            Resel::WireOrangeOff, Resel::WireOrangeOn,
                            Resel::WireSapphireOff, Resel::WireSapphireOn,
                            Resel::WireLimeOff, Resel::WireLimeOn
                        ].contains(&reselboard[x][y]) {
                            // Diagonal neighbors
                            [(1,0), (1, height-1), (0, height-1), (width-1, height-1), (width-1, 0), (width-1, 1), (0, 1), (1,1)]
                        } else if [
                            Resel::AND, Resel::XOR, Resel::Input, Resel::Output
                        ].contains(&reselboard[x][y]) {
                            // Ortho neighbors   --  cheap hack, pad with (0,0)s.
                            [(1,0), (0,height-1), (width-1, 0), (0, 1), (0,0), (0,0), (0,0), (0,0)]
                        } else {
                            // No neighbors. Should not be possible
                            [(0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)]
                        }
                    }.iter() { // for (dx, dy) in ..neighbors to check.. {
                        //print!("({:?} + {:?}, {:?} + {:?}): ", x, dx, y, dy);
                        match (
                            &reselboard[x][y], // Our resel
                            &reselboard[(x + dx) % width][(y + dy)%height] // Neighbor resel
                        ) {
                            // Simple case where the resels match, add the resel to our bag of neighbors
                            (resel_a, resel_b) if (
                                // resel_a matches resel_b, not the originating pixel, and not already visited
                                resel_a == resel_b && *dx != 0 && *dy != 0 && !visited[(x + dx) % width][(y + dy)%height]
                            ) => {
                                neighbors.push(
                                    ((x + dx) % width, (y + dy)%height)
                                );
                                //println!("basic match");
                            },
                            // Wires match, but have different on/off values, add the resel to our bag of neighbors
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
                            // ..., not the originating pixel, and not already visited
                            ) if (*dx != 0 && *dy != 0 && !visited[(x + dx) % width][(y + dy)%height]) => {
                                neighbors.push(
                                    ((x + dx) % width, (y+ dy)%height)
                                );
                                //println!("wire match");
                            },
                            // Else, do nothing
                            (_, _) => {
                                //println!("do nothing");
                            }
                            // note: setting region_by_resel and resel_by_region
                            // happens at the start of this while loop
                      } // match expression to check contiguity;
                        // if contiguous, add to 'neighbors'
                    } // loop which checks adjacent resels for contiguity.
                } // while loop which iterates over all neighbors in a region,
                  // updating the region_by_resel and resels_by_region mappings.
            } // consider resel.
              // if in visited, ignore, look at the next resel.
              // else, this is a new region, let's look for all adjacent resels
        } // for each y
    } // for each x

    (region_by_resel, resels_by_region)
}


fn main() {
    let img = load_image_from_filename("test.png");
    let (width, height) = img.dimensions();
    println!("Dimensions: {}x{}", width, height);
    println!("Reselboard: {:?}", image_to_reselboard(&img));
    let (region_by_resel, resels_by_region) = 
        resel_region_mapping_from_reselboard(&image_to_reselboard(&img), width as usize, height as usize);
    println!("Region by resel:\n{:?}", region_by_resel);
    println!("Resel by region:\n{:?}", resels_by_region);
}

