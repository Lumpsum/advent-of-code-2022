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

#[derive(Debug, Clone)]
enum DirContent<'a> {
    Dir(&'a str),
    File(File<'a>)
}

#[allow(dead_code)]
struct Dir<'a> {
    name: &'a str,
    content: Vec<DirContent<'a>>
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct File<'a> {
    name: &'a str,
    size: usize
}


pub struct FileDirectory<'b> {
    pwd: Vec<&'b str>,
    directories: HashMap<String, Vec<DirContent<'b>>>,
    directory_size: HashMap<String, usize>
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
                let current_pwd = self.pwd.join("/");
                self.directories.insert(current_pwd, values);
                ()
            }
        };
        ()
    }

    fn calculate_directory_sizes(&mut self) {
        let directories = self.directories.clone();
        for dir_content in directories {
            self.calculate_directory_size(dir_content.0);
        }
    }

    fn calculate_directory_size(&mut self, key: String) -> usize {
        if let Some(i) = self.directory_size.get(&key[..]) {
            return *i
        }

        let mut total_size: usize = 0;
        let directories = self.directories.clone();
        let dir_content: &Vec<DirContent> = directories.get(&key[..]).expect("Did not find key");

        for file_content in dir_content {
            let s = match file_content {
                DirContent::Dir(d) => {
                    match self.directory_size.get(&key[..]) {
                        Some(i) => *i,
                        None => self.calculate_directory_size(format!("{}/{}", key, d))
                    }
                },
                DirContent::File(f) => f.size
            };
            total_size += s;
        };

        self.directory_size.insert(key, total_size);
        total_size
    }

    fn get_size_of_directories_below_threshold(&self, threshold: usize) -> usize {
        self.directory_size.iter().filter_map(|item| {
            if item.1 <= &threshold {
                Some(*item.1)
            } else {
                None
            }
        }).sum()
    }
}


impl Solution for DaySeven {
    type Input<'a> = FileDirectory<'a>;
    type Output = usize;

    fn preprocess<'a>(data: &'a str) -> Self::Input<'a> {
        let commands = data.split("$ ").filter_map(|s| Command::try_from(s).ok()).collect::<Vec<Command>>();

        let mut fd = FileDirectory {
            pwd: Vec::new(),
            directories: HashMap::new(),
            directory_size: HashMap::new()
        };
        for cmd in commands {
            fd.parse_command(&cmd);
        };

        fd.calculate_directory_sizes();
        fd
    }

    fn puzzle_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        Some(input.get_size_of_directories_below_threshold(100_000))
    }

    fn puzzle_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        let disk_space_needed: usize = 30_000_000 - (70_000_000 - *input.directory_size.get("/").expect("/ not found!"));
        Some(*input.directory_size.values().into_iter().filter(|&f| f > &disk_space_needed).min().unwrap())
    }
}
