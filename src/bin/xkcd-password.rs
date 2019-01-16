use rand::seq::SliceRandom;
use rand::thread_rng;
use std::process::Command;
use structopt::StructOpt;

const ASPELL_DICT: &str = "en_US";

/// Generate XKCD 936 password
#[derive(StructOpt, Debug)]
struct Opt {
    /// Minimum bits of entropy
    #[structopt(long = "min-bits", default_value = "44")]
    min_bits: u32,
}

fn min_length(num_symbols: u32, min_bits: u32) -> u32 {
    let length = f64::from(min_bits) / f64::from(num_symbols).log2();
    length.ceil() as u32
}

fn get_words() -> Vec<String> {
    let output = Command::new("aspell")
        .args(&["dump", "master", ASPELL_DICT])
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    output
        .lines()
        .filter(|s| s.chars().all(|c| c.is_lowercase()))
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

fn main() {
    let opt = Opt::from_args();
    let min_bits = opt.min_bits;

    let words = get_words();
    let length = min_length(words.len() as u32, min_bits);

    let mut rng = thread_rng();
    let password = (0..length)
        .map(|_| words.choose(&mut rng).unwrap().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", password);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_length() {
        assert_eq!(min_length(2, 160), 160);
        assert_eq!(min_length(10, 160), 49);
        assert_eq!(min_length(36, 160), 31);
        assert_eq!(min_length(95, 160), 25);
    }
}
