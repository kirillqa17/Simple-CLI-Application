use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Error in file path");
    }

    let file_path = &args[1];

    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            panic!("Error reading file: {}", err);
        }
    };

    let trimmed_content = content.trim();

    let word_count = count_words(&trimmed_content);
    let line_count = count_lines(&trimmed_content);
    let char_count = count_characters(&trimmed_content);

    println!("Words: {}", word_count);
    println!("Lines: {}", line_count);
    println!("Characters: {}", char_count);
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

fn count_lines(content: &str) -> usize {
    content.lines().count()
}

fn count_characters(content: &str) -> usize {
    content.chars().filter(|&c| c != '\r').count()
}
