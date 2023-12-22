/// main.rs: Reso CLI
/// 
/// 

use clap::{Parser};


mod resel;
#[allow(dead_code)]
mod reselboard;
#[allow(unused_parens)]
mod regionmap;
mod incidencemap;
#[allow(unused_parens)]
mod resocircuit;

use image::{DynamicImage};

//use resel::{Resel};
use reselboard::{
  ReselBoard,
  load_image_from_filename_string,
};
//use regionmap::{RegionMap};
use resocircuit::{ResoCircuit};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image to simulate, e.g. `reso_logo.png`
    #[arg(short, long)]
    input: String,

    // Output prefix to save to, e.g. `reso_logo_` to save to `reso_logo_%i.png`
    #[arg(short, long)]
    output: String,

    /// Number of simulation steps to run
    #[arg(short, long, default_value_t = 1)]
    numiter: usize,

    /// Verbosity
    #[arg(short, long)]
    verbose: Option<bool>,
}

// Record start and end timing
pub fn main() {
  let args = Args::parse();
  let mut rc = ResoCircuit::from(
    ReselBoard::from(
      load_image_from_filename_string(
        args.input
      ).unwrap()
    )
  );
  let verbose = args.verbose.unwrap_or(false);

  let mut tt_interpolated: String;
  // Index from 1 to N, inclusive. "0" is the input image
  for tt in 1..(args.numiter+1) {
    tt_interpolated = format!(
      "{:0width$}", tt, width=args.numiter.to_string().len()
    );

    if verbose {
      println!("Step {} / {}", tt_interpolated, args.numiter);
    }
    rc.iterate();
    rc.update_pixels();

    rc.get_image().unwrap().save(
      format!("{}{}.png", args.output, tt_interpolated)
    );

  }
}
