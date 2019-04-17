#[macro_use]
extern crate itertools;

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

    let transition: HashMap<(String, String), Vec<String>> = HashMap::new();

    // loop through three words at a time:
    // for w0, w1, w2 in zip(words[0:], words[1:], words[2:]):
    //     transition[w0, w1].append(w2)

    for (w1, w2, w3) in izip!(&words, &words[1..], &words[2..]) {
        // if doesnt exist, create
        // if does exist, append

        let curr = transition.get(&(w1.to_string(), w2.to_string())).unwrap();
        curr.push(w3.to_string());
    }

    println!("{:?}", words);
}
