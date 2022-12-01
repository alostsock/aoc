use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type LineResult = io::Result<io::Lines<io::BufReader<File>>>;

pub fn read_lines<P>(filename: P) -> LineResult
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
