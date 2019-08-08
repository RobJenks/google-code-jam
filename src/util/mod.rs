use std::io::{self, BufRead};

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
    match std::fs::read_to_string(path) {
        Err(e) => panic!("Failed to read \"{}\": {}", path, e),
        Ok(res) => res
    }
}

pub fn read_file_lines(path: &str) -> Vec<String> {
    get_lines(read_file(path))
}
