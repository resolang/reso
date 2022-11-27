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

fn get_region_by_pixel_from_reselboard(
    reselboard: &Vec<Vec<Resel>>,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut region_idx: usize = 0;
    let mut visited:     Vec<Vec<bool>> = vec![vec![false; height as usize]; width as usize];
    let mut region_idxs: Vec<Vec<usize>> = vec![vec![0; height as usize]; width as usize];
    
    for x in 0..width {
        for y in 0..height {
            if visited[x][y] {
                // already visited; skip
            } else {
                // unvisited!
                // let's record this as the start of a new region!
                region_idx += 1;

                // now, let's send out to explore our neighbors!
                let mut neighbors: Vec<(usize, usize)> = Vec::new();
                neighbors.push((x, y));

                // for each neighbor...
                while !neighbors.is_empty() {
                    let (x, y) = neighbors.pop().unwrap();
                    // leave our mark
                    region_idxs[x][y] = region_idx;
                    visited[x][y] = true;

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
                        if !visited[x][y] && !(dx == dy){
                            match (
                                &reselboard[x][y], // Our pixel
                                &reselboard[(x + dx) % width][(y + dy)%height] // Neighbor pixel
                            ) {
                                // Simple case, resels match, neighbor found!
                                (resel_a, resel_b) if (resel_a == resel_b) => neighbors.push(
                                    ((x + dx) % width, (y + dy)%height)
                                ),       
                                // Wires match
                                    (Resel::WireOrangeOff   | Resel::WireOrangeOn,   Resel::WireOrangeOff | Resel::WireOrangeOn)
                                |   (Resel::WireSapphireOff | Resel::WireSapphireOn, Resel::WireSapphireOff | Resel::WireSapphireOn)
                                |   (Resel::WireLimeOff     | Resel::WireLimeOn,     Resel::WireLimeOff | Resel::WireLimeOn)
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
    }

    region_idxs
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
}