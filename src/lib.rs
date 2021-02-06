use std::error::Error;
use std::fs;
use terminal_size::{Width, Height, terminal_size};

//use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub hidden: bool,
    pub path: String,
}

impl Config {
    pub fn new(args: &[String])
        -> Result<Config, &'static str> {
        //println!("Config - args: {:?}", &args);
        if args.len() == 1 {
            Ok(Config {
                hidden: false,
                path: String::from("."),
            })
        } else if args.len() == 2 {
            let path = args[1].clone();
            Ok(Config {
                hidden: false,
                path,
            })
        } else {
            let path = args[2].clone();
            //let hidd_str = args[1].clone();
            Ok(Config {
                hidden: true,
                path,
            })
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    //println!("run    - config:  {:?}", &config);
    let path = &config.path;
    //println!("run    - path: {}", &path);

    let mut content: Vec<String> = vec![];
    for entry in fs::read_dir(&path).unwrap() {
        match entry {
            Ok(direntry) => {
                let filename = direntry.file_name().into_string().unwrap();
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

pub fn maxsize(content: &Vec<String>) -> usize {
    let content_count: Vec<usize> = content.iter().map(|x| x.chars()
                                                            .count()
                                                      ).collect();
    content_count.iter()
                 .fold(content_count[0], |acc, &item| {
        if item > acc {
            item
        } else {
            acc
        }
    })
}

//pub fn adapt_size(content: Vec<String>, max_string_size: usize) -> Vec<String> {
    ////content.iter().map(|s| s
                            //////.push(" ".repeat(max_string_size - s.chars().count()))
        ////)
        ////.collect()

    //let fmt_content: Vec<String> = vec![];
    //for filename in content {
        //filename.push_str(" ".repeat(max_string_size - &filename.chars().count()));
        //fmt_content.push(filename);

    //}
    //fmt_content
//}
//
pub fn pad_filename(filename: &String, width: usize) -> String {
    String::from(format!("{:<width$}", filename, width=width))
}

pub fn term_size() -> u16 {
    let size = terminal_size();
    if let Some((Width(w), Height(_))) = size {
        w
    } else {
        80
    }
}

pub fn stdout(content: Vec<String>) {
    //let mut stdoutput = String::new();
    //for filename in content {
        //stdoutput.push_str(&filename);
        //stdoutput.push_str("\t");
    //}
    let max_string_size = maxsize(&content) + 3;
    let content_formatted: Vec<String> = content
        .iter()
        .map(|s| pad_filename(&s, max_string_size))
        .collect();

    let max_string_size = max_string_size as u16;
    let max_item_per_line = term_size() / max_string_size;
    let mut res = String::new();
    for (index, filename) in content_formatted.iter().enumerate() {
        res.push_str(filename);
        if index as u16 % max_item_per_line == 0 {
            res.push_str("\n");
        }
    }
    println!("{}", res);

    //println!("{}", content_formatted.join(""));

}
