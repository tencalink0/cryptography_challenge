pub trait TextParsing {
    fn get_words(file_contents: String) -> Vec<String>;

    fn get_chars(file_contents: String) -> Vec<char>;
}

pub struct Parser;

impl TextParsing for Parser {
    fn get_words(file_contents: String) -> Vec<String> {
        return file_contents
            .split_whitespace()
            .map(|word| word.to_string())
            .collect();
    }

    fn get_chars(file_contents: String) -> Vec<char> {
        return file_contents
            .chars()
            .collect();
    }
}