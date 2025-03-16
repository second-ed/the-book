use std::fs;
use std::io::{self, Read};

fn main() {
    dbg!(read_str_from_file());
}

fn read_str_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
