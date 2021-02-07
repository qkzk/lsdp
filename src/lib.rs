use std::error::Error;
use std::fs::read_dir;
use std::process;
use std::{collections::HashMap, println};

mod bloc_format;
mod extract;

//pub use crate::extract;

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
        let vec_options: Vec<String> = vec![String::from("a"), String::from("l")];
        //println!("Config - args: {:?}", &args);
        if args.len() == 1 {
            Ok(Config {
                ..Default::default()
            })
        } else {
            let mut map_options = HashMap::new();
            let mut path = String::from(".");
            for opt in vec_options.iter() {
                map_options.insert(opt, false);
            }

            for arg in args[1..].iter() {
                if arg.starts_with("-") {
                    let option = String::from(&arg[1..]);
                    let arg_characs: Vec<&str> = option.split("").collect();
                    for option in vec_options.iter() {
                        if arg_characs.contains(&&option[..]) {
                            map_options.insert(option, true);
                        }
                    }
                } else {
                    path = String::from(arg);
                }
            }
            Ok(Config {
                hidden: map_options[&String::from("a")],
                list: map_options[&String::from("l")],
                path,
                ..Default::default()
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //println!("run    - config:  {:?}", &config);
    //println!("run    - path: {}", &path);
    match config.list {
        true => display_list(config),
        false => display_column(config),
    }
}

pub fn display_list(config: Config) -> Result<(), Box<dyn Error>> {
    //println!("{:?}", config);
    let path = &config.path;
    let mut content_file_info: Vec<extract::FileInfo> = vec![];
    for entry in read_dir(&path)? {
        match entry {
            Ok(direntry) => {
                let filename = extract::extract_filename(&direntry);
                if config.hidden || !filename.starts_with(".") {
                    let file_info = extract::FileInfo::new(&direntry).unwrap_or_else(|err| {
                        eprintln!("Problem accessing file {} information: {}", filename, err);
                        process::exit(1);
                    });
                    content_file_info.push(file_info);
                }
            }
            Err(e) => println!("Something went wrong {}", e),
        }
    }
    stdout_list(content_file_info);
    Ok(())
}

pub fn display_column(config: Config) -> Result<(), Box<dyn Error>> {
    let path = &config.path;
    let mut content: Vec<String> = vec![];
    for entry in read_dir(&path)? {
        match entry {
            Ok(direntry) => {
                let filename = extract::extract_filename(&direntry);
                if config.hidden || !filename.starts_with(".") {
                    content.push(filename);
                }
            }
            Err(e) => println!("Something went wrong {}", e),
        }
    }

    stdout(content);

    Ok(())
}

pub fn stdout_list(content_file_info: Vec<extract::FileInfo>) {
    for file_info in content_file_info.iter() {
        println!(
            "{}{} {} {} {}",
            file_info.dir_symbol,
            file_info.permissions,
            file_info.file_size,
            file_info.owner,
            file_info.filename
        );
    }
}

pub fn stdout(content: Vec<String>) {
    let max_string_size = bloc_format::maxsize(&content) + 3;
    let content_formatted = bloc_format::pad_strings(content, max_string_size);
    let string_output = bloc_format::format_bloc(content_formatted, max_string_size);
    println!("{}", string_output);
}
