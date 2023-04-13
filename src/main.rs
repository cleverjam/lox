use std::fs::File;
use std::io::{BufReader, Read};
use std::process;

use clap::Parser;

use crate::scanner::Scanner;

mod scanner;
mod tokens;

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

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args = Args::try_parse();
    match args {
        Err(e) => {
            let _ = e.print();
            process::exit(64)
        }
        Ok(args) => {
            let file_str = read_file(&args)?;
            let mut scanner = Scanner::new(&file_str);
            scanner.scan();
            Ok(())
        }
    }
}

fn read_file(args: &Args) -> Result<String, Error> {
    match File::open(&args.filename) {
        Err(_) => {
            println!("Fatal error opening file: {}", &args.filename);
            process::exit(66)
        }
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let mut buf = String::new();
            let _ = reader.read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}
