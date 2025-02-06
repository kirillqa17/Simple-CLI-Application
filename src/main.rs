use std::fs;
use std::io;

fn main() {
    let mut user_input = String::new();
    println!("Please enter the absolute path:");
    io::stdin().read_line(&mut user_input).expect("Error: Can't read the line");

    let file_path = user_input.trim();

    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            panic!("Error reading file: {}", err)
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