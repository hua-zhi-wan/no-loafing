use std::env;


pub enum BootArgs {
    None,
    Update(String),
    PathAndLang(String, String)
}

pub fn get_boot_args() -> BootArgs {
    let mut args = env::args();
    let size =  args.len();
    args.next();

    if let Some(argstr) = args.next() {
        match argstr.as_str() {
            "update" => {
                if let Some(url) =  args.next() {
                    return BootArgs::Update(url);
                }
                else {
                    return BootArgs::Update(String::from("https://hanayabuki.github.io/no-loafing"));
                }
            }
            _ => {
                match size {
                    2 => return BootArgs::PathAndLang(
                        argstr,
                        String::from("default")
                    ),
                    3 => return BootArgs::PathAndLang(
                        argstr,
                        args.next().unwrap()
                    ),
                    _ => return BootArgs::None,
                }
            }
        }
    }
    else {
        return BootArgs::None;
    }
}
