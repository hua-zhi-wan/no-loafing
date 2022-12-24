use std::collections::{HashMap, VecDeque};
use std::fs::DirEntry;
use std::fs::{self, DirBuilder, File};
use std::io::BufRead;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use hyper::body::HttpBody;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;

use crate::config::Config;
use crate::jsonparser;

///
///  Read Local Files

pub fn read_dir_as_entry_vec(
    root_path: &PathBuf,
    config: &Config,
) -> Result<Vec<DirEntry>, &'static str> {
    let read_dir = fs::read_dir(root_path).expect("Wrong Path.");
    let mut file_vec = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back(read_dir);
    while let Some(mut dir) = queue.pop_front() {
        while let Some(entry) = dir.next() {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                queue.push_back(fs::read_dir(entry.path()).unwrap());
            } else if file_type.is_file() {
                let file_name = entry.file_name();
                let file_name = file_name.to_str().expect("Illegal File Name.");
                let pats = &config.suffix;
                for pat in pats {
                    if pat.is_match(file_name) {
                        file_vec.push(entry);
                        break;
                    }
                }
            }
        }
    }

    Ok(file_vec)
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
        .expect("Unknown Error When Reading files.")
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

///
/// Load Local Config

fn config_path() -> PathBuf {
    let mut addr = std::env::current_exe().unwrap();
    addr.pop();
    addr.push("config");
    if let Ok(_) = DirBuilder::new().create(&addr) {
        println!("New Config Directory.")
    }

    addr
}

pub fn load_config_info() -> Result<HashMap<String, String>, &'static str> {
    // get config path
    let mut cfg_path = config_path();
    cfg_path.push("main.json");

    // read config info
    let cfg_info_str = fs::read_to_string(&cfg_path);
    if cfg_info_str.is_err() {
        return Err("couldn't read 'config/main.json'.");
    }

    let cfg_info_str = cfg_info_str.unwrap();
    if let Some((_lang_vec, lang_map)) = jsonparser::parse_main_config(&cfg_info_str) {
        return Ok(lang_map);
    } else {
        return Err("couldn't parse json.");
    }
}

pub fn load_config_item(lang_name: &str) -> Result<Config, &'static str> {
    let mut cfg_path = config_path();
    let mut file_name = String::from(lang_name);
    file_name.push_str(".json");
    cfg_path.push(file_name);

    if let Ok(cfg_info_str) = fs::read_to_string(&cfg_path) {
        let json_item = jsonparser::parse_config_item(&cfg_info_str);
        return match json_item {
            Ok(item) => Ok(item),
            Err(_err) => Err("Cannot parse json."),
        };
    } else {
        return Err("Cannot open file.");
    }
}

///
/// Get Online Config
pub async fn update_configs(uri: &str) {
    let mut addr = String::from(uri);
    let mut config_path = config_path();

    // Get main.json
    addr.push_str("/main.json");
    println!("Updating Config from `{}`", &addr);
    println!("Downloading `main.json`...");
    let chunk = get(&addr).await;
    if let Err(err) = &chunk {
        eprintln!("{}", &err);
        process::exit(0);
    }
    let resp_json = String::from_utf8(chunk.unwrap()).unwrap();

    config_path.push("main.json");
    let mut main_json_file = File::create(&config_path).unwrap();
    config_path.pop();
    main_json_file.write_all(resp_json.as_bytes()).unwrap();

    // foreach
    let (lang_iter, _) = jsonparser::parse_main_config(&resp_json).unwrap();
    let lang_iter = lang_iter.iter();
    for lang_name in lang_iter {
        let mut item_file_name = String::from(lang_name);
        item_file_name.push_str(".json");

        addr = String::from(uri);
        addr.push('/');
        addr.push_str(&item_file_name);

        println!("Downloading `{}`...", &item_file_name);
        let item_resp_json = get(&addr).await;
        if let Err(err) = &item_resp_json {
            eprintln!("{}", &err);
        }

        config_path.push(&item_file_name);
        let mut item_json_file = File::create(&config_path).unwrap();
        config_path.pop();
        item_json_file
            .write_all(item_resp_json.unwrap().as_slice())
            .unwrap();
    }

    println!("DONE!");
}

async fn get(uri: &str) -> Result<Vec<u8>, hyper::Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = uri.parse::<Uri>().expect("Wrong URI");
    let mut resp = client.get(uri).await.unwrap();

    match resp.body_mut().data().await.unwrap() {
        Ok(resp) => Ok(resp.to_vec()),
        Err(e) => Err(e),
    }
}
