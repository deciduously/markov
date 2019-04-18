#[macro_use]
extern crate itertools;

use rand::{seq::SliceRandom, thread_rng, Rng};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufReader, Read},
};

fn read_file() -> Result<String, Box<std::error::Error>> {
    let file = OpenOptions::new().read(true).open("poetry.txt")?;
    let mut contents = String::new();
    let mut bfr = BufReader::new(file);
    bfr.read_to_string(&mut contents)?;
    Ok(contents)
}

fn split_words<'a>(s: &'a str) -> Vec<&'a str> {
    let spaces_re = Regex::new(r" +").unwrap();
    spaces_re.split(s).collect::<Vec<&str>>()
}

// you need a HashMap with a default value for missing entries
// this acts as a lookup table
// ([w1, w2], [possiblities])

fn main() {
    let file_str = read_file().unwrap();
    let words = split_words(&file_str);

    let mut transition: HashMap<(String, String), Vec<String>> = HashMap::new();

    // izip!() is from itertools crate
    for (w0, w1, w2) in izip!(&words, &words[1..], &words[2..]) {
        // add w3 to the key (w1, w2)
        let curr = transition
            .entry((w0.to_string(), w1.to_string()))
            .or_insert(vec![]);
        curr.push(w2.to_string());
    }

    // pick a random start between 0 and words.len() - 3
    let mut rng = thread_rng();
    let i = rng.gen_range(0, words.len() - 3);

    // grab the first three words at that location
    let mut w0 = words[i];
    let mut w1 = words[i + 1];
    let mut w2 = words[i + 3];

    // print current word and then a space, and update our words

    for _ in 0..200 {
        print!("{} ", w2);
        // do we need to do this before reassigning?
        w2 = transition
            .get(&(w0.to_string(), w1.to_string()))
            .unwrap()
            .choose(&mut rng)
            .unwrap();
        w0 = w1;
        w1 = w2;
    }
}
