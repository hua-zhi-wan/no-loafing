use std::collections::HashMap;
use json::JsonValue;

use crate::config::Config;


pub fn parse_main_config(cfg_info_str: &str) -> (Vec<String>, HashMap<String, String>) {
    let mut lang_vec = Vec::new();
    let mut lang_map = HashMap::new();
    
    let cfg_info_json = json::parse(&cfg_info_str).unwrap();

    if let JsonValue::Object(cfg_info_json) = cfg_info_json {
        for (lang_name, array) in cfg_info_json.iter() {
            // load supported langs
            if let JsonValue::Array(array) = array { 
                for item in array {
                    if let JsonValue::Short(item) = item {
                        lang_map.insert(String::from(item.as_str()), String::from(lang_name));
                    }
                }
            }
            // out
            lang_vec.push(String::from(lang_name));
        }
    }

    (lang_vec, lang_map)
}

pub fn parse_config_item(cfg_info_str: &str) -> Option<Config> {
    let cfg_info_json = json::parse(&cfg_info_str).unwrap();

    let this_name;
    let mut this_suffix = Vec::new();
    let mut this_ignore = Vec::new();

    if let JsonValue::Object(json) = cfg_info_json {
        this_name = json.get("name").unwrap().as_str().unwrap();
        if let JsonValue::Array(json) = json.get("suffix").unwrap() {
            for item in json.iter() {
                if let JsonValue::Short(item) = item {
                    this_suffix.push(item.as_str());
                }
            }
        }
        if let JsonValue::Array(json) = json.get("ignore").unwrap() {
            for item in json.iter() {
                if let JsonValue::Short(item) = item {
                    this_ignore.push(item.as_str());
                }
            }
        }

        return Some(
            Config::new(this_name, this_suffix, this_ignore)
        )
    }

    None
}