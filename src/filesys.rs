use std::collections::VecDeque;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::io::BufRead;

use crate::boot::BootArgs;
use crate::config::Config;

pub fn read_dir_as_entry_vec(boot_args: &BootArgs) -> Vec<DirEntry> {
    let read_dir = fs::read_dir(&boot_args.root_path).expect("Wrong Path.");
    let mut file_vec = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back(read_dir);
    while !queue.is_empty() {
        let mut dir = queue.pop_front().unwrap();
        while let Some(entry) = dir.next() {
            let entry = entry.expect("Wrong Entry");
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                queue.push_back(fs::read_dir(entry.path()).unwrap());
            } else if file_type.is_file() {
                let file_name = entry.file_name();
                let file_name = file_name.to_str().expect("Illegal File Name.");
                let pats = &boot_args.config.suffix;
                for pat in pats {
                    if pat.is_match(file_name) {
                        file_vec.push(entry);
                        break;
                    }
                }
            }
        }
    }

    file_vec
}

pub fn read_file_by_lines(entry: &DirEntry, config: &Config) -> (i32, i32) {
    let file = fs::File::open(entry.path()).unwrap();
    let mut buf_reader = io::BufReader::new(file);

    let mut line_str = String::new();

    let mut i = 0;
    let mut i_ignore = 0;
    let ignores = &config.ignore;

    while buf_reader
        .read_line(&mut line_str)
        .expect("Unknown Error When Reading file.")
        != 0
    {
        let mut flag = false;
        for re in ignores {
            if re.is_match(&line_str) {
                flag = true;
                i_ignore += 1;
                break;
            }
        }
        if !flag {
            i += 1;
        }
        line_str.clear();
    }

    (i, i_ignore)
}
