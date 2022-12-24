#[allow(unused)]
use std::collections;

use regex::Regex;

pub struct Config {
    pub name: String,
    pub suffix: Vec<Regex>,
    pub ignore: Vec<Regex>,
}

impl Config {
    pub fn new(name: &str, suffix: Vec<&str>, ignore: Vec<&str>) -> Config {
        let mut _suffix = Vec::new();
        let mut _ignore = Vec::new();
        for item in suffix {
            _suffix.push(Regex::new(item).expect("Wrong Regex in Suffix."));
        }
        for item in ignore {
            _ignore.push(Regex::new(item).expect("Wrong Regex in Ignore."));
        }

        Config {
            name: String::from(name),
            suffix: _suffix,
            ignore: _ignore,
        }
    }
}
