mod utils;
mod text;
mod frequency;

use utils::{read_file};
use text::{Parser, TextParsing};
use frequency::{Unit, Frequency};

fn main() {
    let file = read_file(String::from("dummy_text.txt"));
    let chars_vec = Parser::get_chars(file);
    println!("{:?}", chars_vec);
    let freq = Frequency::<char>::new(chars_vec);
    let frequencies = freq.get_frequencies(
        Unit::Char,
        true,
    );
    println!("{:?}", frequencies);
}
