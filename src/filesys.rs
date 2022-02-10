use std::collections::{VecDeque, HashMap};
use std::fs::{self, File};
use std::fs::DirEntry;
use std::io::{self, Write};
use std::io::BufRead;
use std::process;
use std::path::PathBuf;

use hyper::body::HttpBody;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;

use crate::config::Config;
use crate::output::error_handler;
use crate::{jsonparser, output};

///
///  Read Local Files

pub fn read_dir_as_entry_vec(root_path: &PathBuf, config: &Config) -> Vec<DirEntry> {
    let read_dir = fs::read_dir(root_path).expect("Wrong Path.");
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

///
/// Load Local Config

fn config_path() -> PathBuf {
    /*
    let mut addr = std::env::current_exe().unwrap();
    addr.pop();
    addr.push("/config");
    addr.clone()
    */ 
    PathBuf::from("d:\\Github\\tmp")
}

pub fn load_config_info() -> Option<HashMap<String, String>>{
    // get config path
    let mut cfg_path = config_path();
    cfg_path.push("main.json");

    // read config info
    let cfg_info_str = fs::read_to_string(&cfg_path);
    if cfg_info_str.is_err() {
        return None;
    }

    let cfg_info_str = cfg_info_str.unwrap();
    let (_lang_vec, lang_map) = jsonparser::parse_main_config(&cfg_info_str);

    Some(lang_map)
}

pub fn load_config_item(lang_name: &str) -> Option<Config>{
    let mut cfg_path = config_path();
    let mut file_name = String::from(lang_name);
    file_name.push_str(".json");
    cfg_path.push(file_name);

    if let Ok(cfg_info_str) = fs::read_to_string(&cfg_path) {
        return jsonparser::parse_config_item(&cfg_info_str);
    }
    else {
        return None;
    }
}


///
/// Get Online Config
pub async fn update_configs(uri: &str) {
    let mut addr = String::from(uri);
    let mut config_path = config_path();

    
    // Get main.json
    addr.push_str("/main.json");
    println!("Downloading `main.json` from `{}`...", &addr);
    let chunk = get(&addr).await;
    if let Err(err) = &chunk {
        output::error_handler("UPDATE-GET-MAIN", &err.to_string());
        process::exit(0);
    }
    let resp_json = String::from_utf8(chunk.unwrap()).unwrap();

    config_path.push("main.json");
    let mut main_json_file = File::create(&config_path).unwrap();
    config_path.pop();
    main_json_file.write_all(resp_json.as_bytes()).unwrap();

    
    // foreach
    let (lang_iter, _) = jsonparser::parse_main_config(&resp_json);
    let lang_iter = lang_iter.iter();
    for lang_name in lang_iter {
        let mut item_file_name = String::from(lang_name);
        item_file_name.push_str(".json");

        addr = String::from(uri);
        addr.push('/');
        addr.push_str(&item_file_name);

        println!("Downloading `{}` from `{}`...", &item_file_name, &addr);
        let item_resp_json = get(&addr).await;
        if let Err(err) = &item_resp_json {
            error_handler("UPDATE-GET-ITEM", &err.to_string());
        }

        config_path.push(&item_file_name);
        let mut item_json_file = File::create(&config_path).unwrap();
        item_json_file.write_all(item_resp_json.unwrap().as_slice()).unwrap();
    }
}

async fn get(uri: &str) -> Result<Vec<u8>, hyper::Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = uri.parse::<Uri>().expect("Wrong URI");
    let mut resp = client.get(uri).await.unwrap();

    match resp.body_mut().data().await.unwrap() {
        Ok(resp) => Ok(resp.to_vec()),
        Err(e) => Err(e)
    }
}

#[tokio::test]
pub async fn test() {
    update_configs("http://hanayabuki.github.io/no-loafing/main.json").await;
}
