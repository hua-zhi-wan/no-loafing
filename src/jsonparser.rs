use json::JsonValue;
use std::collections::HashMap;

use crate::config::Config;

pub fn parse_main_config(cfg_info_str: &str) -> Option<(Vec<String>, HashMap<String, String>)> {
    let mut lang_vec = Vec::new();
    let mut lang_map = HashMap::new();

    let cfg_info_json = json::parse(cfg_info_str).unwrap();

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

    Some((lang_vec, lang_map))
}

pub fn parse_config_item(cfg_info_str: &str) -> Result<Config, &'static str> {
    let cfg_info_json = json::parse(cfg_info_str).unwrap();

    if let JsonValue::Object(json) = cfg_info_json {
        let this_name = json.get("name").unwrap().as_str().unwrap();
        let mut this_suffix = Vec::new();
        let mut this_ignore = Vec::new();

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

        return Ok(Config::new(this_name, this_suffix, this_ignore));
    }

    Err("")
}
