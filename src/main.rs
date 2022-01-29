mod boot;
mod help;
mod filesys;
mod config;
mod output;

use std::process;

fn main() {
    match boot::get_boot_args() {
        None => {
            println!("{}", help::HELP_INFO);
            process::exit(0);
        },
        Some(boot) => {
            let entry_vec = filesys::read_dir_as_entry_vec(&boot);
            let mut i_total = 0;
            for entry in entry_vec {
                print!("{:#?}\t", entry.file_name());
                let (i, i_ig) = filesys::read_file_by_lines(&entry, &boot.config);
                println!("----\t{} line{}. ({} ignored)", i, if i>1 {"s"} else {""}, i_ig);
                i_total += i;
            }
            println!("{} lines in total.\n", i_total);
        }
    }
}
