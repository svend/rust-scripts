use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
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

fn main() -> Result<()> {
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

fn has_newline<R: io::Read + io::Seek>(mut r: R) -> io::Result<bool> {
    let mut buffer = String::new();
    if r.seek(SeekFrom::End(-1)).is_err() {
        // seek -1 on empty reader returns "invalid seek to a negative or
        // overflowing position"
        return Ok(false);
    };
    r.read_to_string(&mut buffer)?;

    match buffer.chars().last() {
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

        let cursor = io::Cursor::new("\n");
        assert!(has_newline(cursor).unwrap());

        let cursor = io::Cursor::new("one\two");
        assert!(!has_newline(cursor).unwrap());

        let cursor = io::Cursor::new("");
        assert!(!has_newline(cursor).unwrap());
    }
}
