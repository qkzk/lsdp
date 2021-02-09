use std::fs::read_dir;
use std::process;
use std::error::Error;

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

pub fn display_error(msg: &str) {
    eprintln!("Problem accessing file information: {}", msg);
    process::exit(1);
}

pub fn display_list(config: config::Config) -> Result<(), Box<dyn Error>> {
    stdout_list(
        read_dir(&config.path)?
            .map(|entry| {
                extract::FileInfo::new(&entry.unwrap()).unwrap_or_else(|err| {
                    eprintln!("Problem accessing file information: {}", err);
                    process::exit(1);
                })
            })
            .filter(|fileinfo| config.hidden || !fileinfo.filename.starts_with("."))
            .collect(),
    );

    Ok(())
}

pub fn display_column(config: config::Config) -> Result<(), Box<dyn Error>> {
    stdout(
        read_dir(&config.path)?
            .map(|entry| extract::extract_filename(&entry.unwrap()))
            .filter(|filename| config.hidden || !filename.starts_with("."))
            .collect(),
    );
    Ok(())
}

pub fn stdout_list(content: Vec<extract::FileInfo>) {
    for file_info in content.iter() {
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
