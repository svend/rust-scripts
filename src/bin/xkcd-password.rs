use rand::seq::SliceRandom;
use rand::thread_rng;
use structopt::StructOpt;

/// Generate XKCD 936 password
#[derive(StructOpt, Debug)]
struct Opt {
    /// Minimum bits of entropy
    #[structopt(long = "min-bits", default_value = "44")]
    min_bits: u32,
}

fn main() {
    let opt = Opt::from_args();
    let min_bits = opt.min_bits;

    let words = get_words();
    let length = min_length(words.len() as u32, min_bits);

    let mut rng = thread_rng();
    let password = std::iter::repeat_with(|| words.choose(&mut rng).unwrap().to_string())
        .take(length as usize)
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", password);
}

fn get_words() -> Vec<String> {
    let words = include_str!("bip-0039-english.txt");
    words.lines().map(|s| s.to_string()).collect()
}

fn min_length(num_symbols: u32, min_bits: u32) -> u32 {
    let length = f64::from(min_bits) / f64::from(num_symbols).log2();
    length.ceil() as u32
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
