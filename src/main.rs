use std::{
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
    s.split_whitespace().collect::<Vec<&str>>()
}

fn main() {
    let file_str = read_file().unwrap();
    let words = split_words(&file_str);
    println!("{:?}", words);
}
