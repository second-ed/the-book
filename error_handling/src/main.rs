use std::fs;
use std::io::{self};

fn main() {
    match read_str_from_file() {
        Ok(file) => {
            dbg!(file);
        }
        Err(err) => {
            dbg!(err);
        }
    }
}

fn read_str_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
