use std::collections::HashMap;

use regex::Regex;

pub struct Config {
    pub name: String,
    pub identity: Vec<Regex>,
    pub suffix: Vec<Regex>,
    pub ignore: Vec<Regex>,
}

impl Config {
    pub fn new(name: &str, identity: Vec<&str>, suffix: Vec<&str>, ignore: Vec<&str>) -> Config {
        let mut _identity = Vec::new();
        let mut _suffix = Vec::new();
        let mut _ignore = Vec::new();
        for item in identity {
            _identity.push(Regex::new(&item).expect("Wrong Regex in Identity."));
        }
        for item in suffix {
            _suffix.push(Regex::new(&item).expect("Wrong Regex in Suffix."));
        }
        for item in ignore {
            _ignore.push(Regex::new(&item).expect("Wrong Regex in Ignore."));
        }

        Config {
            name: String::from(name),
            identity: _identity,
            suffix: _suffix,
            ignore: _ignore,
        }
    }
}

pub fn auto_config(name: &str) -> Config {
    match name {
        "java" => Config::new(
            "java",
            vec!["\\.java$"],
            vec!["\\.java$"],
            vec![
                // blank line
                "^\\s*$",
                // only comment sign
                "^\\s*//\\s*$",
                // import statement
                "^import",
            ],
        ),
        _ => Config::new(
            "auto",
            Vec::new(),
            vec![
                // c/c++
                "\\.h$",
                "\\.c(pp)?$",
                // java
                "\\.java$",
                // c#
                "\\.cs$",
                // python
                "\\.py$",
                "\\.ipynb$",
                // Web front-end
                "\\.js$",
                "\\.html?$",
                "\\.css$",
                // others(WwWwwW)
                "\\.go$",
                "\\.kt$",
                "\\.rs$",
            ],
            vec!["^\\s*$"],
        ),
    }
}

#[tokio::test]
pub async fn update_configs() {
    let response = reqwest::get("https://httpbin.org/ip")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap();
    println!("{:?}", response);
}
