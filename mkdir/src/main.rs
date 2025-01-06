extern crate clap;
use std::{
    collections::LinkedList,
    fs::{create_dir, set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser)]
#[clap(version, author)]
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

    /// Set the mode of the created directories to the specified mode
    #[arg(short, long, default_value = "755")]
    mode: String,
}

fn create(dir: &str, parents: bool, verbose: bool, mode: u32) {
    if !parents {
        let path = Path::new(&dir);
        _create(path, verbose, mode);
        return;
    }

    let mut path = PathBuf::from(dir);
    let mut to_create: LinkedList<PathBuf> = LinkedList::new();

    while !path.exists() && !(path == Path::new(&"")) {
        to_create.push_front(path.clone());
        if !path.pop() {
            break;
        }
    }

    for p in to_create {
        if _create(&p, verbose, mode) != 0 {
            return;
        }
    }
}

fn _create(path: &Path, verbose: bool, mode: u32) -> u8 {
    if let Err(e) = create_dir(path) {
        eprintln!("Cannot create directory \"{}\": {}", path.display(), e);
        return 1;
    }
    if verbose {
        println!("Created directory '{}'", path.display());
    }
    if let Err(e) = set_permissions(path, Permissions::from_mode(mode)) {
        eprintln!(
            "Cannot set permissions for directory \"{}\": {}",
            path.display(),
            e
        );
        return 1;
    }

    return 0;
}

fn main() {
    let args = Args::parse();
    // if args.version {
    //     println!("mkdir v{}", VERSION);
    //     println!("Written by {}", AUTHORS);
    //     return;
    // }
    if args.directires.is_empty() {
        eprintln!("Missing [DIRECTORIES]...");
        eprintln!("Try 'mkdir --help' for more information.");
        return;
    }

    let mode = u32::from_str_radix(&args.mode, 8).expect("Invalid mode");

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
        create(&dir, args.parents, args.verbose, mode);
    }
}
