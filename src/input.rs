use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file_path(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("Unable to open file {}: {}", display, err),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("Couldn't read {}: {}", display, err),
        Ok(_) => (),
    }

    return s;
}

pub fn read_file() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Invalid usage: expected file path");
    }

    let path = Path::new(&args[1]);
    read_file_path(&path)
}
