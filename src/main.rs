#[macro_use]
extern crate itertools;

use rand::{seq::SliceRandom, thread_rng, Rng};
use regex::Regex;
use std::{
    collections::HashMap, error::Error, fs::OpenOptions, io::Read, path::PathBuf, str::FromStr,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "markov")]
struct Opt {
    /// Input text file
    #[structopt(short = "i", long = "input")]
    input: Option<PathBuf>,
    /// Output length
    #[structopt(short = "l", long = "length")]
    length: Option<u32>,
}

fn read_file(filename: PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// is there a way not to allocate this Vec?
fn split_words(w: &str) -> Vec<&str> {
    let spaces_re = Regex::new(r" +").unwrap();
    spaces_re.split(w).collect::<Vec<&str>>()
}

fn build_table(words: Vec<&str>) -> HashMap<(&str, &str), Vec<&str>> {
    let mut ret = HashMap::new();
    for (w0, w1, w2) in izip!(&words, &words[1..], &words[2..]) {
        // add w2 to the key (w0, w1)
        let current = ret.entry((*w0, *w1)).or_insert_with(Vec::new);
        current.push(*w2);
    }
    ret
}

// Unused - just to demonstrate
fn _build_table_no_itertools(words: Vec<&str>) -> HashMap<(&str, &str), Vec<&str>> {
    let mut ret = HashMap::new();
    let all_words = &words[..];
    let offset_1 = &words[1..];
    let offset_2 = &words[2..];
    for (w0, w1, w2) in all_words
        .iter()
        .zip(offset_1.iter())
        .zip(offset_2.iter())
        .map(|((x, y), z)| (x, y, z))
    {
        // add w2 to the key (w0, w1)
        let current = ret.entry((*w0, *w1)).or_insert_with(Vec::new);
        current.push(*w2);
    }
    ret
}

fn run(input: PathBuf, length: u32) -> Result<(), Box<dyn Error>> {
    // read file and build lookup table
    let file_str = read_file(input)?;
    let words = split_words(&file_str);

    // pick a start location
    // pick a random start between 0 and words.len() - 3
    let mut rng = thread_rng();
    let i = rng.gen_range(0, words.len() - 3);

    // grab the first three words at that location
    let mut w0 = words[i];
    let mut w1 = words[i + 1];
    let mut w2 = words[i + 2];

    // build the lookup table - takes ownership of words!
    let lookup = build_table(words);

    // each iteration, print current word and then a space, and update our words
    for _ in 0..length {
        // append to output
        print!("{} ", w2);

        // choose the next word
        w2 = &lookup[&(w0, w1)].choose(&mut rng).unwrap_or(&"NONE");
        w0 = w1;
        w1 = w2;
    }

    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let filename = opt
        .input
        .unwrap_or_else(|| PathBuf::from_str("poetry.txt").unwrap());
    let length = opt.length.unwrap_or(350);

    if let Err(e) = run(filename, length) {
        eprintln!("Error: {}", e);
        ::std::process::exit(1);
    };
}
