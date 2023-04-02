use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about)]
#[command(help_template("\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args}{after-help}
"))]
struct Args {
    /// Source file
    #[clap(value_name = "input file")]
    filename: String,
    /// Output file
    #[clap(short,long, value_name = "output file", default_value = "output")]
    output: String
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
