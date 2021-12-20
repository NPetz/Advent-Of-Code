use std::{fs, path::Path};

pub fn read_input(text: &str) -> String {
    let path_string = format!("src/{}/input.txt", text);
    fs::read_to_string(Path::new(&path_string)).unwrap()
}
