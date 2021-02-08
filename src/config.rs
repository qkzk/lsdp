use std::collections::HashMap;
use std::env;


#[derive(Debug)]
pub struct Config {
    pub hidden: bool, // "-a"
    pub list: bool,   // "-l"
    pub path: String, // "/usr/bin"
}
impl Default for Config {
    fn default() -> Self {
        Config {
            hidden: false,
            list: false,
            path: String::from("."),
        }
    }
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, &'static str> {

        match args.len() {
            1 => Ok(Config{ ..Default::default() }),
            _ => {
                let (path, map_options) = parse_args(args);
                Ok(Config {
                    hidden: map_options[&String::from("a")],
                    list: map_options[&String::from("l")],
                    path,
                    ..Default::default()
                })
            }
        }
    }
}

pub fn default_map_options() -> ([String;2], HashMap<String, bool>) {
    let arr_options: [String; 2] = [String::from("a"), String::from("l")];
    let mut map_options = HashMap::new();
    for opt in arr_options.iter() {
        map_options.insert(String::from(opt), false);
    }
    (arr_options, map_options)
}

pub fn parse_args(mut args: env::Args) -> (String, HashMap<String, bool>) {
    let mut path = String::from(".");
    let (arr_options, mut map_options) = default_map_options();
    args.next();
    for arg in args {
        if arg.starts_with("-") {
            let option = String::from(&arg[1..]);
            let arg_characs: Vec<&str> = option.split("").collect();
            for carac in &arg_characs {
                if arr_options.contains(&String::from(*carac)) {
                    map_options.insert(String::from(*carac), true);
                }
            }
        } else {
            path = String::from(arg);
        }
    }
    (path, map_options)
}
