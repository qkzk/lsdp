use std::fs;

fn main() {
    let mut content: Vec<String> = vec![];
    for entry in fs::read_dir(".").unwrap() {
        match entry {
            Ok(direntry) => {
                let thing = direntry.file_name().into_string().unwrap();
                content.push(thing);
            },
            Err(e) => println!("Something went wrong {}", e),
        }
    }

    let mut stdoutput = String::new();
    for name in &content {
        stdoutput.push_str(&name);
        stdoutput.push_str(" ");
    }
    println!("{}", stdoutput);
}
