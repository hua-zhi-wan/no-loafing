use std::env;


pub enum BootArgs {
    None,
    Update(String),
    PathAndLang(String, String)
}

pub fn get_boot_args() -> BootArgs {
    let mut args = env::args();
    args.next();
    let size =  args.len();

    match args.next().unwrap().as_str() {
        "update" => {
            if let Some(url) =  args.next() {
                return BootArgs::Update(url);
            }
            else {
                return BootArgs::Update(String::from("http://hanayabuki.github.io/no-loafing"));
            }
        }
        _ => {
            match size {
                2 => return BootArgs::PathAndLang(
                    args.next().unwrap(),
                    String::from("default")
                ),
                3 => return BootArgs::PathAndLang(
                    args.next().unwrap(),
                    args.next().unwrap()
                ),
                _ => return BootArgs::None,
            }
        }
    }
}
