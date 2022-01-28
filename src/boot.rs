use std::env;

use crate::config::{self, Config};

pub struct BootArgs {
    pub root_path: String,
    pub config: Config,
}

pub fn get_boot_args() -> Option<BootArgs> {
    let mut args = env::args();
    let size =  args.len();

    match size {
        2 => Some(BootArgs {
            root_path: args.next().unwrap(),
            config: config::auto_config(""),
        }),
        3 => Some(BootArgs {
            root_path: args.next().unwrap(),
            config: config::auto_config(&args.next().unwrap()),
        }),
        _ => None,
    }
}
