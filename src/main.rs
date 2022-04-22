use clap::Parser;
use std::{fs::File, io::{BufReader, BufRead}};
use anyhow::Result;
use regex::Regex;
use crypt3::crypt;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    file: std::path::PathBuf,
}

fn main() {
  main_error().unwrap();
}

fn main_error() -> Result<()> {
    let args = Args::parse();
    let regex = Regex::new(r"(\w+):((\$\d\$[\w]+)\$([./\d\w]+)|\*|\!):")?;

    let file = File::open(args.file)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
      let new_line = line?;
      let unprocessed_result = regex.captures(&new_line);

      if let Some(unprocessed_result) = unprocessed_result {
        let username = unprocessed_result.get(1).unwrap().as_str();
        let encrypted_password = unprocessed_result.get(2).unwrap().as_str();

        match encrypted_password {
          "!" => println!("User {} is a locked user account", username),
          "*" => println!("User {} is a locked servie account", username),
          pswd => {
            let digest = crypt(b"abcdefghijklmnop", b"$1$");
            //println!("{}", pswd);
          }
        }
      } else {
        println!("Bad format: {}", new_line);
      }
    }

    Ok(())
}
