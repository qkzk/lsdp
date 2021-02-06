use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub hidden: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        //println!("Config - args: {:?}", &args);
        if args.len() == 1 {
            Ok(Config {
                path: String::from("."),
                hidden: false,
            })
        } else if args.len() == 2 {
            let path = args[1].clone();
            Ok(Config {
                path,
                hidden: false,
            })
        } else {
            let path = args[2].clone();
            //let hidd_str = args[1].clone();
            Ok(Config {
                path,
                hidden: true,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //println!("run    - config:  {:?}", &config);
    let path = &config.path;
    //println!("run    - path: {}", &path);

    let mut content: Vec<String> = vec![];
    for entry in fs::read_dir(&path).unwrap() {
        match entry {
            Ok(direntry) => {
                let thingname = direntry.file_name().into_string().unwrap();
                if config.hidden || !thingname.starts_with(".") {
                    content.push(thingname);
                }
            },
            Err(e) => println!("Something went wrong {}", e),
        }
    }

    let mut stdoutput = String::new();
    for thingname in &content {
        stdoutput.push_str(&thingname);
        stdoutput.push_str(" ");
    }
    println!("{}", stdoutput);

    Ok(())
}
