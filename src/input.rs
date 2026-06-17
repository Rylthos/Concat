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
