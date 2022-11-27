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
    let mut reselboard = vec![vec![Resel::new(); height as usize]; width as usize];
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let resel = Resel::from_rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
            reselboard[x as usize][y as usize] = resel;
        }
    }
    reselboard
}

enum Resel {
    WireOrangeOff, WireOrangeOn,
    WireSapphireOff, WireSapphireOn,
    WireLimeOff, WireLimeOn,
    AND, XOR, 
    Input, Output,
    Empty
}

impl Resel {
    fn new() -> Resel {
        Resel::Empty
    }

    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Resel {
        match (r, g, b, a) {
            (128,  64,   0, 255) => Resel::WireOrangeOff,
            (255, 128,   0, 255) => Resel::WireOrangeOn,
            (  0,  64, 128, 255) => Resel::WireSapphireOff,
            (  0, 128, 255, 255) => Resel::WireSapphireOn,
            (64,  128,   0, 255) => Resel::WireLimeOff,
            (128, 256,   0, 255) => Resel::WireLimeOn,
            (  0, 128,  64, 255) => Resel::AND,
            (  0, 255, 128, 255) => Resel::XOR,
            ( 64,   0, 128, 255) => Resel::Input,
            (128,   0, 255, 255) => Resel::Output,
            _ => Resel::Empty
        }
    }

    fn to_rgba(&self) -> Rgba<u8> {
        match self {
            Resel::WireOrangeOff   => Rgba([128,  64,   0, 255]),
            Resel::WireOrangeOn    => Rgba([255, 128,   0, 255]),
            Resel::WireSapphireOff => Rgba([  0,  64, 128, 255]),
            Resel::WireSapphireOn  => Rgba([  0, 128, 255, 255]),
            Resel::WireLimeOff     => Rgba([64,  128,   0, 255]),
            Resel::WireLimeOn      => Rgba([128, 256,   0, 255]),
            Resel::AND             => Rgba([  0, 128,  64, 255]),
            Resel::XOR             => Rgba([  0, 255, 128, 255]),
            Resel::Input           => Rgba([ 64,   0, 128, 255]),
            Resel::Output          => Rgba([128,   0, 255, 255]),
            Resel::Empty           => Rgba([0, 0, 0, 0])
        }
    }
}

fn main() {
    let img = load_image_fron_filename("test.png");
    let (width, height) = img.dimensions();
    println!("{}x{}", width, height);
}