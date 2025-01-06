use std::{
    fs::File,
    io::{stdout, Write},
    path::{Path, PathBuf},
};

use zip::write::SimpleFileOptions;

extern crate clap;
extern crate zip;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// FIles or directories to archive
    paths: Vec<PathBuf>,

    /// Output file path
    #[arg(short, long, default_value = "Archive.zip")]
    output_path: PathBuf,
}

fn check_if_paths_is_not_empty(paths: &[PathBuf]) -> Result<(), &'static str> {
    if paths.is_empty() {
        return Err("No paths provided");
    }
    Ok(())
}

fn archive(output: &Path, fp: &Path) {
    let out = File::create(output).unwrap();
    let mut zip = zip::ZipWriter::new(out);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file(fp.file_name().unwrap().to_str().unwrap(), options)
        .unwrap();

    let mut file = File::open(fp).unwrap();
    std::io::copy(&mut file, &mut zip).unwrap();
    zip.finish().unwrap();
}

fn main() {
    let mut args = Args::parse();
    if let Err(e) = check_if_paths_is_not_empty(&args.paths) {
        eprintln!("{}", e);
        return;
    }

    if args.output_path.exists() && args.output_path.is_file() {
        print!(
            "Output file \"{}\" already exists, overwrite? [y]es, [n]o, [N]ew output file: ",
            args.output_path.display()
        );
        stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
            "y" => {}
            "n" => return,
            "N" => {
                print!("Enter new output file path: ");
                stdout().flush().unwrap();
                buf.clear();
                std::io::stdin().read_line(&mut buf).unwrap();
                args.output_path = PathBuf::from(buf.trim());
            }
            _ => {
                eprintln!("Invalid input");
                return;
            }
        }
    }
    archive(&args.output_path, &args.paths[0]);
}
