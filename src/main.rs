use std::{io, process};

mod scanner;
mod tokens;

use crate::scanner::Scanner;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser, Debug, Default)]
#[command(author, version, about)]
#[command(help_template(
    "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args}{after-help}
"
))]
struct Args {
    /// Source file
    #[clap(value_name = "input file")]
    filename: String,
    /// Output file
    #[clap(short, long, value_name = "output file", default_value = "output")]
    output: String,
}

fn main() {
    let args = Args::try_parse();
    match args {
        Err(e) => {
            let _ = e.print();
            process::exit(64)
        }
        Ok(args) => {
            let _ = read_file(&args);
        }
    }
}

fn read_file(args: &Args) -> Result<(), io::Error> {
    match File::open(&args.filename) {
        Err(_) => {
            println!("Fatal error opening file: {}", &args.filename);
            process::exit(66)
        }
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let scanner = Scanner::new(&mut reader);
            scanner.scan();
        }
    }
    Ok(())
}
