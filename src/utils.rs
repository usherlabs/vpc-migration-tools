use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use sysinfo::Disk;

pub fn read_file_to_string(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    lines.join("\n")
}


pub struct Option {
    pub label: String,
    pub value: String,
}

impl Display for Option {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.label, self.value)
    }
}
