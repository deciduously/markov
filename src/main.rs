#[macro_use]
extern crate itertools;

use rand::{seq::SliceRandom, thread_rng, Rng};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufReader, Read},
};

fn read_file(filename: &str) -> Result<String, Box<std::error::Error>> {
    let file = OpenOptions::new().read(true).open(filename)?;
    let mut contents = String::new();
    let mut bfr = BufReader::new(file);
    bfr.read_to_string(&mut contents)?;
    Ok(contents)
}

fn split_words<'a>(w: &'a str) -> Vec<&'a str> {
    let spaces_re = Regex::new(r" +").unwrap();
    spaces_re.split(w).collect::<Vec<&str>>()
}

fn run(input: &str, length: u32) -> Result<(), Box<std::error::Error>> {
    let file_str = read_file(input)?;
    let words = split_words(&file_str);

    let mut transition: HashMap<(&str, &str), Vec<&str>> = HashMap::new();

    // izip!() is from itertools crate
    for (w0, w1, w2) in izip!(&words, &words[1..], &words[2..]) {
        // add w3 to the key (w1, w2)
        let curr = transition.entry((w0, w1)).or_insert_with(Vec::new);
        curr.push(w2);
    }

    // pick a random start between 0 and words.len() - 3
    let mut rng = thread_rng();
    let i = rng.gen_range(0, words.len() - 3);

    // grab the first three words at that location
    let mut w0 = words[i];
    let mut w1 = words[i + 1];
    let mut w2 = words[i + 3];

    // print current word and then a space, and update our words

    for _ in 0..length {
        print!("{} ", w2);
        // do we need to do this before reassigning?
        w2 = &transition[&(w0, w1)].choose(&mut rng).unwrap();
        w0 = w1;
        w1 = w2;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run("poetry.txt", 200) {
        eprintln!("Error: {}", e);
        ::std::process::exit(1);
    };
}
