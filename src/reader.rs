use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use crate::constants::DATA_FOLDER;


pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


pub fn lines_from_file_in_groups(filename: impl AsRef<Path>, group_size: usize) -> Vec<Vec<String>> {
    let lines = lines_from_file(filename);

    let mut group_lines: Vec<Vec<String>> = Vec::new();
    for chunk in lines.chunks(group_size) {
        let mut new_group: Vec<String> = Vec::new();
        for value in chunk {
            new_group.push(value.to_string())
        }
        group_lines.push(new_group)
    }
    group_lines
}


pub struct Reader {}

impl Reader {
    pub fn read(&self, day: usize) -> std::io::Result<String> {
        let file = File::open(format!("{}day_{}.txt", DATA_FOLDER, day))?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}