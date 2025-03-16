use std::fs::File;
use std::io::{self, Read};

fn main() {
    dbg!(read_str_from_file());
}

fn read_str_from_file() -> Result<String, io::Error> {
    let mut res = String::new();
    File::open("hello.txt")?.read_to_string(&mut res)?;
    Ok(res)
}
