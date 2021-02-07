use fs::{metadata, DirEntry};
use std::error::Error;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::{collections::HashMap, println};
use terminal_size::{terminal_size, Width};
use users::get_user_by_uid;
use std::process;


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
        /*
         * lsdp -la path
         * lsdp -la
         * lsdp -l -a
         * lsdp -l -a path
         * lsdp path
         * lsdp -a -l path
         *
         *
         * Si un argument : pas d'option, pas de path
         * Si
         */
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
                    println!("{}", option);
                    let arg_characs: Vec<&str> = option.split("").collect();
                    println!("characs : {:?}", arg_characs);
                    for option in vec_options.iter() {
                        println!("option : {}", option);
                        if arg_characs.contains(&&option[..]) {
                            println!("found option {}", option);
                            map_options.insert(option, true);
                        }
                    }
                } else {
                    path = String::from(arg);
                }
            }
            println!("map options {:?}", map_options);
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

pub struct FileInfo {
    pub filename: String,
    pub file_size: String,
    pub dir_symbol: String,
    pub permissions: String,
    pub owner: String,
}

impl FileInfo {
    pub fn new(direntry: &DirEntry) -> Result<FileInfo, &'static str> {
        let filename = extract_filename(&direntry);
        let file_size = human_size(extract_file_size(&direntry));
        let dir_symbol = extract_dir_symbol(&direntry);
        let permissions = extract_permissions_string(&direntry);
        let owner = extract_username(&direntry);

        Ok(FileInfo {
            filename,
            file_size,
            dir_symbol,
            permissions,
            owner,
        } )
    }
}

pub fn display_list(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config);
    let path = &config.path;
    //let mut content: Vec<(String, String, String, String, String)> = vec![];
    let mut content_file_info: Vec<FileInfo> = vec![];
    for entry in fs::read_dir(&path)? {
        match entry {
            Ok(direntry) => {
                let filename = extract_filename(&direntry);
                if config.hidden || !filename.starts_with(".") {
                    let file_info = FileInfo::new(&direntry).unwrap_or_else(|err| {
                        eprintln!("Problem accessing file {} information: {}",
                            filename, err);
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

pub fn extract_filename(direntry: &DirEntry) -> String {
    direntry.file_name().into_string().unwrap()
}

pub fn extract_permissions_string(direntry: &DirEntry) -> String {
    let mode = metadata(direntry.path()).unwrap().permissions().mode() & 511;

    let mode_o = mode >> 6;
    let mode_g = (mode >> 3) & 7;
    let mode_a = mode & 7;

    let s_o = convert_octal_mode(mode_o);
    let s_g = convert_octal_mode(mode_g);
    let s_a = convert_octal_mode(mode_a);

    [s_o, s_g, s_a].join("")
}

pub fn convert_octal_mode(mode: u32) -> String {
    let rwx  = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];
    String::from(rwx[(mode & 7 as u32) as usize])
}

pub fn extract_username(direntry: &DirEntry) -> String {
    let meta = metadata(direntry.path());
    let owner_id: u32 = meta.unwrap().uid();
    let user = get_user_by_uid(owner_id).unwrap();
    String::from(user.name().to_str().unwrap())
}

pub fn extract_dir_symbol(direntry: &DirEntry) -> String {
    let meta = metadata(direntry.path());
    if meta.unwrap().is_dir() {
        String::from("d")
    } else {
        String::from(".")
    }
}

pub fn extract_file_size(direntry: &DirEntry) -> u64 {
    direntry.path().metadata().unwrap().len()
}

pub fn human_size(bytes: u64) -> String {
    let size = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];
    let factor = (bytes.to_string().chars().count() as u64 - 1) / 3  as u64;
    let human_size = format!("{:>3}{:<1}",
                bytes / (1204 as u64).pow(factor as u32),
                size[factor as usize]
            );
    human_size
}

pub fn stdout_list(content_file_info: Vec<FileInfo>) {
    //let first_line = "Permissions Size User    Date Modified  Name";
    //println!("{}", first_line);
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

pub fn display_column(config: Config) -> Result<(), Box<dyn Error>> {
    let path = &config.path;
    let mut content: Vec<String> = vec![];
    for entry in fs::read_dir(&path)? {
        match entry {
            Ok(direntry) => {
                let filename = extract_filename(&direntry);
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
    let content_count: Vec<usize> = content.iter().map(|x| x.chars().count()).collect();
    content_count.iter().fold(
        content_count[0],
        |acc, &item| {
            if item > acc {
                item
            } else {
                acc
            }
        },
    )
}

pub fn pad_filename(filename: &String, width: usize) -> String {
    String::from(format!("{:<width$}", filename, width = width))
}

pub fn term_size() -> u16 {
    if let Some((Width(w), _)) = terminal_size() {
        w
    } else {
        80
    }
}

pub fn pad_strings(content: Vec<String>, max_string_size: usize) -> Vec<String> {
    content
        .iter()
        .map(|s| pad_filename(&s, max_string_size))
        .collect()
}

pub fn format_bloc(content_formatted: Vec<String>, max_string_size: usize) -> String {
    let max_string_size = max_string_size as u16;
    let max_item_per_line = term_size() / max_string_size;
    let mut string_output = String::new();
    for (index, filename) in content_formatted.iter().enumerate() {
        string_output.push_str(filename);
        if index as u16 % max_item_per_line == 0 {
            string_output.push_str("\n");
        }
    }
    string_output
}

pub fn stdout(content: Vec<String>) {
    let max_string_size = maxsize(&content) + 3;
    let content_formatted = pad_strings(content, max_string_size);
    let string_output = format_bloc(content_formatted, max_string_size);
    println!("{}", string_output);
}
