mod utils;
mod text;
mod frequency;

use utils::{read_file};
use text::{Parser, TextParsing};
use frequency::{Unit, Frequency};

fn main() {
    let file = read_file(String::from("dummy_text.txt"));
    let corpus = Parser::get_chars(file);
    let comparison_text = Parser::get_chars(
        String::from("The cow jumped over the moon")
    );
    println!("{:?}", corpus);
    let freq = Frequency::<char>::new(corpus, comparison_text);
    let fitness = freq.fitness(
        Unit::Char,
        true,
    );
    println!("{}", fitness);
}
