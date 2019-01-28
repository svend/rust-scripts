use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

/// Ensure files contain terminal newline
#[derive(StructOpt, Debug)]
struct Opt {
    /// Write missing newline to files
    #[structopt(long = "write")]
    write: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn append_newline(p: &PathBuf) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(p)?;
    writeln!(file, "")
}

fn has_newline(p: &PathBuf) -> io::Result<bool> {
    let mut file = File::open(&p)?;
    let mut s = String::new();

    file.read_to_string(&mut s)?;

    match s.chars().last() {
        Some('\n') => Ok(true),
        _ => Ok(false),
    }
}

fn try_main() -> io::Result<()> {
    let opt = Opt::from_args();

    for path in opt.files {
        if !has_newline(&path)? {
            println!("{}", path.display());
            if opt.write {
                append_newline(&path)?;
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
