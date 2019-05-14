use std::fs::{File, OpenOptions};
use std::io;
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

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn try_main() -> io::Result<()> {
    let opt = Opt::from_args();

    for path in opt.files {
        let file = File::open(&path)?;
        if !has_newline(&file)? {
            println!("{}", path.display());
            if opt.write {
                append_newline(&path)?;
            }
        }
    }

    Ok(())
}

fn has_newline<R: io::Read>(mut r: R) -> io::Result<bool> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;

    match s.chars().last() {
        Some('\n') => Ok(true),
        _ => Ok(false),
    }
}

fn append_newline(p: &Path) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(p)?;
    writeln!(file)
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn test_has_newline() {
        let cursor = io::Cursor::new("one\two\n");
        assert!(has_newline(cursor).unwrap());

        let cursor = io::Cursor::new("one\two");
        assert!(!has_newline(cursor).unwrap());
    }
}
