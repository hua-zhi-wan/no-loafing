mod boot;
mod config;
mod filesys;
mod help;
mod jsonparser;
mod output;

use std::{process, path::PathBuf};

use boot::BootArgs;

fn main() {
    // Init
    let boot_args = boot::get_boot_args();

    // Update
    if let BootArgs::Update(url) = boot_args {
        // Sync
        tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(
            filesys::update_configs(&url)
        );
        process::exit(0);
    }
    // local load
    let lang_map = filesys::load_config_info().unwrap();

    match boot_args {
        BootArgs::None => {
            println!("{}", help::HELP_INFO);
            process::exit(0);
        }
        BootArgs::PathAndLang(dir, lang_tag) => {
            // args operation
            let dir = dir.parse::<PathBuf>().unwrap();
            let lang_config = filesys::load_config_item(&lang_map[&lang_tag]).unwrap();

            let entry_vec = filesys::read_dir_as_entry_vec(&dir, &lang_config);
            let mut i_total = 0;

            let mut max_len: usize = 0;
            for entry in entry_vec.iter() {
                if entry.file_name().is_ascii() && max_len < entry.file_name().len() {
                    max_len = entry.file_name().len();
                }
            }

            for entry in entry_vec.iter() {
                print!("{:<max_len$}\t", entry.file_name().into_string().unwrap(), max_len = max_len);
                let (i, i_ig) = filesys::read_file_by_lines(&entry, &lang_config);
                println!(
                    "|{:>5} line{} |{:>4} ignored.",
                    i,
                    if i > 1 {"s."} else {". "},
                    i_ig
                );
                i_total += i;
            }
            println!("{} lines in total.", i_total);
        }
        _ => {}
    }
}
