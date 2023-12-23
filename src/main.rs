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
use std::time::{SystemTime, UNIX_EPOCH, SystemTimeError, Duration};

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

    /// Output prefix to save frames, e.g. `out_` saves to `out_01.png`.
    #[arg(short, long)]
    output: Option<String>,

    /// Number of simulation steps to run
    #[arg(short, long, default_value_t = 1)]
    numiter: usize,

    /// Print to console
    #[arg(short, long)]
    verbose: bool,

}

// Record start and end timing
pub fn main() {
  let args = Args::parse();
  if args.verbose {
    println!("Compiling {}", args.input)
  }
  let start_compile = SystemTime::now();
  let mut rc = ResoCircuit::from(
    ReselBoard::from(
      load_image_from_filename_string(
        args.input.clone()
      ).unwrap()
    )
  );
  if args.verbose {
    print_duration(SystemTime::now().duration_since(start_compile));
  }


  let mut tt_interpolated: String;
  let start_time = SystemTime::now();

  if args.verbose {
    println!("Simulating {} iterations on {}", args.numiter, &args.input);
  }

  // Index from 1 to N, inclusive. "0" is the input image
  for tt in 1..(args.numiter+1) {
    tt_interpolated = format!(
      "{:0width$}", tt, width=args.numiter.to_string().len()
    );

    if (tt % 100) == 0 && args.verbose {
      println!("Step {} of {}", tt_interpolated, args.numiter);
    }
    rc.iterate();

    if args.output.is_some() {
      rc.update_pixels();
      rc.get_image().unwrap().save(
        format!(
          "{}{}.png",
          args.output.as_ref().unwrap_or(&String::from("output_")),
          tt_interpolated
        )
      );
    }   
  }

  // Print time
  if args.verbose {
    print_duration(SystemTime::now().duration_since(start_time));
  }
}

fn print_duration(duration: Result<Duration, SystemTimeError>) {
  match duration {
    Ok(duration) => {
      if duration.as_secs() >= 10 {
        println!("Done in {} s", duration.as_secs());
      } else if duration.as_millis() >= 10 {
        println!("Done in {}ms", duration.as_millis());
      } else if duration.as_micros() >= 10 {
        println!("Done in {}us", duration.as_micros());
      } else if duration.as_nanos() >= 10 {
        println!("Done in {}ns", duration.as_nanos());
      } else if duration.is_zero() {
        println!("Done instantly. Wow! This should be impossible.");
      } else {
        println!("Done in almost an insant.");
      }
    },
    Err(_) => {
      println!("Done in a time machine. Our timer says you went back in time.");
      println!("How did you do that??");
    }
  }
}

/*
todos:
- better verbosity
- save images optional
- 
*/

// eof