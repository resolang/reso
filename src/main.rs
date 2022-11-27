use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage, DynamicImage};

fn load_image_fron_filename(filename: &str) -> DynamicImage {
    // Load the image from the file (copilot)
    let img = image::open(filename).expect("File not found, sorry!");
    let (width, height) = img.dimensions();
    println!("Loaded {} ({}x{} px).", &filename, width, height);
    img
}

fn main() {
    let img = load_image_fron_filename("test.png");
    let (width, height) = img.dimensions();
    println!("{}x{}", width, height);
}