use std::fs::File;
use std::io::{Read};


fn main() {
    let file = File::open("../input/y2022/d7-test.aoc.txt").unwrap();
    let mut string = String::new();
    std::io::BufReader::new(file).read_to_string(&mut string).unwrap();


}