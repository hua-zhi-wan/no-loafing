mod boot;
mod config;
mod filesys;
mod help;
mod jsonparser;

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
    let lang_map = filesys::load_config_info().unwrap_or_else(|err| {
        eprintln!("Error: opening 'config/main.json' failed.\n\t{}", &err);
        process::exit(1);
    });

    match boot_args {
        BootArgs::None => {
            help::help();
            process::exit(0);
        }
        BootArgs::PathAndLang(dir, lang_tag) => {
            // args operation
            let dir = dir.parse::<PathBuf>().unwrap_or_else(|_| {
                eprintln!("Error: parsing directory '{}' failed.\n\tillegal directory.", &dir);
                process::exit(2);
            });

            let lang_config = filesys::load_config_item(&lang_map[&lang_tag]).unwrap_or_else(|err| {
                eprintln!("Error: opening 'config/main.json' failed.\n\t{}", &err);
                process::exit(3);
            });

            let entry_vec = filesys::read_dir_as_entry_vec(&dir, &lang_config).unwrap_or_else(|err| {
                eprintln!("Error: opening directory failed.\n\t{}", &err);
                process::exit(4);
            });
            let mut i_total = 0;

            let mut file_vec = Vec::new();
            let mut max_len: usize = 0;
            let mut max_width_1 = 0;
            let mut max_width_2 = 0;

            for entry in entry_vec.iter() {
                let (i, i_ig) = filesys::read_file_by_lines(&entry, &lang_config);

                let item = (
                    entry.file_name().into_string().unwrap_or_else(|osstr| {
                        format!("{:?}", osstr)
                    }),
                    i.to_string(),
                    i_ig.to_string(),
                );

                if max_len < entry.file_name().len() {
                    max_len = entry.file_name().len();
                }
                max_width_1 = if max_width_1 > item.1.len() { max_width_1 } else { item.1.len() };
                max_width_2 = if max_width_2 > item.2.len() { max_width_2 } else { item.2.len() };
                
                file_vec.push(item);
                i_total += i;
            }

            for file_info in file_vec {
                println!(
                    " {:<max_len$}\t| {:>max_width_1$} lines. | {:>max_width_2$} ignored.", 
                    file_info.0, 
                    file_info.1, 
                    file_info.2,

                    max_len = max_len,
                    max_width_1 = max_width_1 as usize,
                    max_width_2 = max_width_2 as usize,
                );
            }

            println!("\n {} lines in total.", i_total);
        }
        _ => {}
    }
}
