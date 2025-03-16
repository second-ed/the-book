use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_res = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file: {:?}", error);
            })
        } else {
            panic!("problem opening the file {:?}", error);
        }
    });
}
