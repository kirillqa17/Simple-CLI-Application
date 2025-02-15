use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path =match args.len(){
        2 => &args[1],
        _ => panic!("Error in file path"),
    };

    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            panic!("Error reading file: {}", err);
        }
    };

    let trimmed_content = content.trim();

    let word_count = count_words(trimmed_content);
    let line_count = count_lines(trimmed_content);
    let char_count = count_characters(trimmed_content);

    println!("Words: {}", word_count);
    println!("Lines: {}", line_count);
    println!("Characters: {}", char_count);
}

fn is_valid_word_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || c.is_alphabetic()
}

fn count_words(content: &str) -> usize {
    let mut word_counter: usize=0;
    let mut in_word = false;
    for c in content.chars(){
        if is_valid_word_char(c) && !in_word{
            in_word = true;
            word_counter+=1;
        }
        else if c == ' ' || c == '\n' || c == '\t' || c == '\r'{
            in_word = false;
        }
    }
    word_counter
}

fn count_lines(content: &str) -> usize {
    content.lines().count()
}

fn count_characters(content: &str) -> usize {
    content.chars().filter(|&c| c != '\r' && c!= '\n').count()
}
