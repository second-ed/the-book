use std::fs::File;
use std::io::{self, Read};

fn main() {
    dbg!(read_str_from_file());
}

fn read_str_from_file() -> Result<String, io::Error> {
    let str_file_res = File::open("hello.txt");

    let mut str_file = match str_file_res {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut res = String::new();

    match str_file.read_to_string(&mut res) {
        Ok(_) => Ok(res),
        Err(e) => Err(e),
    }
}
