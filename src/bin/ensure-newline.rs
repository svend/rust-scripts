use failure::{Error, ResultExt};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

/// Check files for terminal newline
#[derive(StructOpt, Debug)]
struct Opt {
    /// Write missing newline to files
    #[structopt(long = "write")]
    write: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str), required = true)]
    files: Vec<PathBuf>,
}

fn append_newline(p: &Path) -> Result<(), Error> {
    let mut file = OpenOptions::new().write(true).append(true).open(p)?;
    writeln!(file, "")?;

    Ok(())
}

fn has_newline(p: &Path) -> Result<bool, Error> {
    let mut file = File::open(&p)?;
    let mut s = String::new();

    file.read_to_string(&mut s)?;

    match s.chars().last() {
        Some('\n') => Ok(true),
        _ => Ok(false),
    }
}

fn try_main() -> Result<(), Error> {
    let opt = Opt::from_args();

    for path in opt.files {
        if !has_newline(&path).context("foo")? {
            println!("{}", path.display());
            if opt.write {
                append_newline(&path).context("foo")?;
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
