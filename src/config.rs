use std::collections::HashMap;

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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        //println!("Config - args: {:?}", &args);
        if args.len() == 1 {
            Ok(Config {
                ..Default::default()
            })
        } else {
            let (path, map_options) = parse_args(&args);
            Ok(Config {
                hidden: map_options[&String::from("a")],
                list: map_options[&String::from("l")],
                path,
                ..Default::default()
            })
        }
    }
}

pub fn parse_args(args: &[String]) -> (String, HashMap<String, bool>) {
    let vec_options: Vec<String> = vec![String::from("a"), String::from("l")];
    let mut map_options = HashMap::new();
    let mut path = String::from(".");
    for opt in vec_options.iter() {
        map_options.insert(String::from(opt), false);
    }

    for arg in args[1..].iter() {
        if arg.starts_with("-") {
            let option = String::from(&arg[1..]);
            let arg_characs: Vec<&str> = option.split("").collect();
            for option in vec_options.iter() {
                if arg_characs.contains(&&option[..]) {
                    map_options.insert(String::from(option), true);
                }
            }
        } else {
            path = String::from(arg);
        }
    }
    (path, map_options)
}
