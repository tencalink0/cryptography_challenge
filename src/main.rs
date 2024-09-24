mod utils;
mod text;
mod frequency;

use utils::{read_file};
use text::{Parser, TextParsing};
use frequency::{Unit, Frequency};

fn main() {
    let file = read_file(String::from("dummy_text.txt"));
    let comparison_text_raw = String::from("The cow jumped over the moon");
    
    let corpus = Parser::get_chars(file);
    let comparison_text = Parser::get_chars(comparison_text_raw);
    println!("{:?}", corpus);
    let freq = Frequency::<char>::new(corpus, comparison_text);
    let fitness = freq.fitness(
        Unit::Char,
        true,
    );
    let refined_fitness = fitness * (100.0 as f64);
    println!("FITNESS: {}%", refined_fitness);
}
