use std::io::{self, BufRead, Read};

pub fn stdin_get() -> String {
    let stdin = io::stdin();
    let line = stdin.lock()
        .lines()
        .next().unwrap().unwrap();
    line
}

pub fn stdin_all() -> Vec<String> {
    let stdin = io::stdin();
    let lines = stdin.lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    lines
}

pub fn get_lines(input: String) -> Vec<String> {
    input.split("\n").map(|x| x.to_string()).collect::<Vec<String>>()
}

pub fn read_file(path : &str) -> String {
    let mut file = std::fs::File::open(path).expect("Failed to open input file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read from input file");
    contents

    // Not supported in Google Code compiler version
    /*match std::fs::read_to_string(path) {
        Err(e) => panic!("Failed to read \"{}\": {}", path, e),
        Ok(res) => res
    }*/
}

pub fn read_file_lines(path: &str) -> Vec<String> {
    get_lines(read_file(path))
}


pub struct Input {
    pre_loaded: bool,
    data: Vec<String>,
    read_point: usize
}

impl Input {
    pub fn create(pre_load: bool, data_path: &str) -> Input {
        Input { pre_loaded: pre_load, data:
            if pre_load { read_file_lines(data_path) } else { vec![] }, read_point: 0 }
    }

    pub fn get_line(&mut self) -> String {
        self.next().unwrap_or_else(|| panic!("Failed to read input line"))
    }

    pub fn get_line_as<T>(&mut self) -> T
        where T: std::str::FromStr
    {
        self.get_line().parse::<T>().unwrap_or_else(|_| panic!("Failed to parse input line to desired type"))
    }
}

impl Iterator for Input {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.pre_loaded {
            false => Some(stdin_get()),
            true => {
                if self.read_point == self.data.len() { None }
                else {
                    let line = self.data[self.read_point].clone();
                    self.read_point += 1;
                    Some(line)
                }
            }

        }
    }
}