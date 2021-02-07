use terminal_size::{terminal_size, Width};

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
