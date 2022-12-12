use std::{fmt::Debug, collections::HashMap};

use crate::solution::Solution;

pub struct DaySeven {}


#[derive(Debug, Clone)]
pub enum Command<'a> {
    ChangeDir(&'a str),
    ListDir(Vec<&'a str>),
}

impl <'a>TryFrom<&'a str> for Command<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let cmd = match s.split_whitespace().next() {
            Some(i) => i,
            None => return Err("Empty String".to_string())
        };

        match cmd {
            "cd" => Ok(Command::ChangeDir(s.split_whitespace().last().unwrap())),
            "ls" => {
                Ok(Command::ListDir(s.lines().collect::<Vec<&str>>()[1..].to_vec()))
            },
            _ => panic!("Invalid command") 
        }
    }
}

#[derive(Debug)]
enum DirContent<'a> {
    Dir(&'a str),
    File(File<'a>)
}

struct Dir<'a> {
    name: &'a str,
    content: Vec<DirContent<'a>>
}

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: usize
}


struct FileDirectory<'b> {
    pwd: Vec<&'b str>,
    directories: HashMap<&'b str, Vec<DirContent<'b>>>,
    directory_size: HashMap<&'b str, usize>
}

impl <'b>FileDirectory<'b> {
    fn parse_command(&mut self, cmd: &Command<'b>) {
        match cmd {
            Command::ChangeDir(cd) => {
                match cd {
                    &"/" => {
                        self.pwd.clear();
                        self.pwd.push("/");
                    },
                    &".." => {self.pwd.pop();},
                    c => {self.pwd.push(c.clone());}
                };
            },
            Command::ListDir(ld) => {
                let values = ld.iter().map(|&content| {
                    let split_str = Vec::from_iter(content.split_whitespace());
                    match split_str[0] {
                        "dir" => DirContent::Dir(split_str[1]),
                        _ => DirContent::File(File {name: split_str[1], size: split_str[0].parse::<usize>().unwrap()})
                    }
                }).collect::<Vec<DirContent>>();
                self.directories.insert(&self.pwd.last().unwrap(), values);
                ()
            }
        };
        ()
    }

    fn calculate_directory_size(&mut self, content: (&&str, &Vec<DirContent>)) {
        
    }
}


impl Solution for DaySeven {
    type Input<'a> = Vec<Command<'a>>;
    type Output = usize;

    fn preprocess<'a>(data: &'a str) -> Self::Input<'a> {
        data.split("$ ").filter_map(|s| Command::try_from(s).ok()).collect::<Vec<Command>>()
    }

    fn puzzle_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        let mut fd = FileDirectory {
            pwd: Vec::new(),
            directories: HashMap::new(),
            directory_size: HashMap::new()
        };
        for cmd in input {
            fd.parse_command(cmd);
        };

        None
    }

    fn puzzle_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        None
    }
}
