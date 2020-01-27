use clap::arg_enum;
use rand::seq::SliceRandom;
use rand::thread_rng;
use structopt::StructOpt;

/// Generate XKCD 936 password
#[derive(StructOpt, Debug)]
struct Opt {
    /// Word list
    #[structopt(long = "word-list", default_value = "Bip39")]
    word_list: WordList,
    /// Minimum bits of entropy
    #[structopt(long = "min-bits", default_value = "44")]
    min_bits: usize,
}

arg_enum! {
    #[derive(Debug)]
    enum WordList {
        Bip39,
        EffLarge,
    }
}

impl WordList {
    fn words(&self) -> Vec<String> {
        let words = match self {
            Self::Bip39 => include_str!("bip-0039-english.txt"),
            Self::EffLarge => include_str!("eff_large_wordlist.txt"),
        };
        words.lines().map(|s| s.to_string()).collect()
    }
}

fn main() {
    let opt = Opt::from_args();
    let min_bits = opt.min_bits;

    let words = WordList::Bip39.words();
    let length = min_length(words.len(), min_bits);

    let mut rng = thread_rng();
    let password = std::iter::repeat_with(|| {
        words
            .choose(&mut rng)
            .expect("word list is empty")
            .to_string()
    })
    .take(length)
    .collect::<Vec<_>>()
    .join(" ");

    println!("{}", password);
}

fn min_length(num_symbols: usize, min_bits: usize) -> usize {
    let length = min_bits as f64 / (num_symbols as f64).log2();
    length.ceil() as usize
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
