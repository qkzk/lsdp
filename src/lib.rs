use std::error::Error;
use std::fs::read_dir;
use std::process;

mod bloc_format;
pub mod config;
mod extract;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    //println!("run    - config:  {:?}", &config);
    //println!("run    - path: {}", &path);
    match config.list {
        true => display_list(config),
        false => display_column(config),
    }
}

pub fn display_list(config: config::Config) -> Result<(), Box<dyn Error>> {
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

pub fn display_column(config: config::Config) -> Result<(), Box<dyn Error>> {
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
