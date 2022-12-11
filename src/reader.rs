use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use crate::constants::DATA_FOLDER;


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