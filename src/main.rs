use image::{GenericImageView, ImageResult, ImageBuffer, Rgba, RgbaImage, DynamicImage};

fn load_image_from_filename(filename: &str) -> DynamicImage {
    // Load the image from the file (copilot)
    let img = image::open(filename).expect("File not found, sorry!");
    let (width, height) = img.dimensions();
    println!("Loaded {} ({}x{} px).", &filename, width, height);
    img
}

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
    // TODO! This can be a fixed size array, I think.
    // see https://stackoverflow.com/questions/59164456/
}

//todo: resoascii_to_resoboard, resoboard_to_resoascii
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
//todo: rewrite with static allocs where possible

fn pixel_region_mapping_from_reselboard(
    reselboard: &Vec<Vec<Resel>>,
    width: usize,
    height: usize,
) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {
    let mut region_idx: usize = 0;
    let mut visited:     Vec<Vec<bool>> = vec![vec![false; height as usize]; width as usize];
    let mut region_by_pixel: Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
    let mut pixels_by_region: Vec<Vec<(usize, usize)>> = vec![Vec::new()];

    // todo: This can run class-by-region as well.
    // (... And all the elements in the compilation step!)
    // (
    // Wire nodes: Set as off by default, to "on" if we see one

    // TODO FROM HERE:
    // 1. refactor 'region_by_pixel' to 'region_by_pixel'
    
    // For each pixel
    for x in 0..width {
        for y in 0..height {
            if visited[x][y] {
                // Ignore it if already visited!
            } else {
                // Pixel is not visited -- this pixel marks a new region!
                // First, update our region count, and prepare a new list of region pixels to populate
                // (region_idx 0 is skipped intentionally, and pixels_by_region[0] should stay empty)
                region_idx += 1;
                pixels_by_region.push(Vec::new());

                // Now, let's explore this pixel and all of our neighbors one-by-one
                let mut neighbors: Vec<(usize, usize)> = Vec::new();
                neighbors.push((x, y));

                // For each neighbor, until we run out:
                while !neighbors.is_empty() {
                    let (x, y) = neighbors.pop().unwrap();

                    // Record info about the newly-inducted pixel to our region!
                    region_by_pixel[x][y] = region_idx; // Mark this region on our map
                    visited[x][y] = true; // Mark it as visited
                    // TODO
                    pixels_by_region[region_idx].push((x.clone(),y.clone())); // Remember this pixel belongs to this region

                    // Check contiguity
                    for (dx, dy) in {
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
                            // Ortho neighbors   --  cheap hack, pad with (0,0)s. (matched out of execution below)
                            [(1,0), (0,height), (width, 0), (0, 1), (0,0), (0,0), (0,0), (0,0)]
                        } else {
                            // No neighbors
                            [(0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)]
                        }
                    }.iter() { // for (dx, dy) in ..neighbors to check.. {
                        match (
                            &reselboard[x][y], // Our pixel
                            &reselboard[(x + dx) % width][(y + dy)%height] // Neighbor pixel
                        ) {
                            // Simple case, resels match, neighbor found!
                            (resel_a, resel_b) if (resel_a == resel_b && dx != dy) => neighbors.push(
                                ((x + dx) % width, (y + dy)%height)
                            ),
                            // Wires match
                                (Resel::WireOrangeOff   | Resel::WireOrangeOn,   Resel::WireOrangeOff | Resel::WireOrangeOn)
                            |   (Resel::WireSapphireOff | Resel::WireSapphireOn, Resel::WireSapphireOff | Resel::WireSapphireOn)
                            |   (Resel::WireLimeOff     | Resel::WireLimeOn,     Resel::WireLimeOff | Resel::WireLimeOn)
                            if dx != dy
                            => neighbors.push(
                                ((x + dx) % width, (y + dy)%height)
                            ),
                            // Else, do nothing
                            (_, _) => ()
                        }
                    }
                }
            }
        }
    }

    (region_by_pixel, pixels_by_region)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Resel {
    WireOrangeOff,   WireOrangeOn,
    WireSapphireOff, WireSapphireOn,
    WireLimeOff,     WireLimeOn,
    AND,   XOR, 
    Input, Output,
    Empty
}

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

fn main() {
    let img = load_image_from_filename("test.png");
    let (width, height) = img.dimensions();
    println!("{}x{}", width, height);
    println!("{:?}", image_to_reselboard(&img));
    println!("{:?}", pixel_region_mapping_from_reselboard(&image_to_reselboard(&img), width as usize, height as usize));
}