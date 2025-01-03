extern crate clap;
extern crate zip;

use clap::Parser;

#[derive(Parser)]
struct Args {
    paths: Vec<String>,
    // #[arg(short, long)]
    // output: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.paths.is_empty() {
        eprintln!("No paths provided");
        return;
    }
}
