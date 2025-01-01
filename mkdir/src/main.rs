extern crate clap;
use std::{
    collections::LinkedList,
    fs::create_dir,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Parser)]
struct Args {
    /// Directories to create
    directires: Vec<String>,

    /// No error if existing, make parent directories as needed,
    /// with their file modes unaffected by any -m option
    #[arg(short, long, default_value_t = false)]
    parents: bool,

    /// Print a message for each created directory
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Print version information and exit
    #[arg(long, default_value_t = false)]
    version: bool,
}

fn create(dir: &str, parents: bool, verbose: bool) {
    if !parents {
        let path = Path::new(&dir);
        _create(path, verbose);
        return;
    }

    let from_root = dir.starts_with('/');
    let mut parts = dir.split('/').collect::<Vec<&str>>();
    if parts.is_empty() {
        return;
    }

    if parts[0] == "" {
        parts = parts[1..].to_vec();
    }

    let mut to_create: LinkedList<PathBuf> = LinkedList::new();

    for i in (0..parts.len()).rev() {
        let strpath: String;
        if from_root {
            strpath = "/".to_string() + &parts[0..=i].join("/");
        } else {
            strpath = parts[0..=i].join("/");
        }

        let path = PathBuf::from_str(&strpath).unwrap();
        if path.exists() {
            break;
        }
        to_create.push_front(path);
    }
    for p in to_create {
        if _create(&p, verbose) != 0 {
            return;
        }
    }
}

fn _create(path: &Path, verbose: bool) -> u8 {
    if let Err(e) = create_dir(path) {
        eprintln!("Cannot create directory \"{}\": {}", path.display(), e);
        return 1;
    }
    if verbose {
        println!("Created directory '{}'", path.display());
    }
    return 0;
}

fn main() {
    let args = Args::parse();
    if args.version {
        println!("mkdir v{}", VERSION);
        println!("Written by {}", AUTHORS);
        return;
    }
    if args.directires.is_empty() {
        eprintln!("mkdir: missing operand");
        eprintln!("Try 'mkdir --help' for more information.");
        return;
    }

    for dir in args.directires {
        let path = Path::new(&dir);
        if !args.parents {
            if let Some(parent) = path.parent() {
                if !parent.exists() && !(parent == Path::new(&"")) {
                    eprintln!(
                        "Cannot create directory \"{}\": No such file or directory",
                        parent.display()
                    );
                    return;
                }
            }
        }
        create(&dir, args.parents, args.verbose);
    }
}
