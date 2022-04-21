use clap::Parser;
use std::{fs::read_to_string, io::BufReader};
use anyhow::Result;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    file: std::path::PathBuf,
}

fn main() {
  main_error().unwrap();
}

fn main_error() -> Result<()> {
    let args = Args::parse();

    let reader = BufReader::new(args.file);
    let lines: Vec<_> = reader.lines().collect();

  for l in lines {
      ...
  }

    Ok(())
}
