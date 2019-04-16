use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
};

// step one - read in and separate into words
fn read_words() -> Result<Vec<String>, Box<std::error::Error>> {
    let file = OpenOptions::new().read(true).open("poetry.txt")?;
    let mut contents = String::new();
    let mut bfr = BufReader::new(file);
    bfr.read_to_string(&mut contents)?;
    let ret: Vec<&str> = contents.split_whitespace().collect();
    let owned_ret: Vec<String> = ret.iter().map(|s| s.to_string()).collect();
    Ok(owned_ret)
}

fn main() {
    println!("{:?}", read_words());
}
