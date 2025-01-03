use std::fs::File;

use zip::write::SimpleFileOptions;

extern crate clap;
extern crate zip;

// use clap::Parser;

// #[derive(Parser)]
// struct Args {
//     paths: Vec<String>,
// }

fn archive_cargo_toml() {
    let out = File::create("archive.zip").unwrap();
    let mut zip = zip::ZipWriter::new(out);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file("Cargo.toml", options).unwrap();

    let mut file = File::open("Cargo.toml").unwrap();
    std::io::copy(&mut file, &mut zip).unwrap();
    zip.finish().unwrap();
}

fn main() {
    // let args = Args::parse();
    // if args.paths.is_empty() {
    //     eprintln!("No paths provided");
    //     return;
    // }
    archive_cargo_toml();
}
