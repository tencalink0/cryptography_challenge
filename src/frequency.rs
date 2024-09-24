use std::any::type_name;
use std::any::Any;

use std::collections::HashMap;

pub enum Unit {
    Char,
    Word,
}

pub enum ComparisonChoice {
    Corpus,
    Text
}

fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

#[derive(Debug, Clone)]
pub struct Frequency<T> {
    corpus: Vec<T>,
    comparison_text: Vec<T>,
}

impl Frequency<char> { // Impl trait must have the derive methods included
    pub fn new(corpus: Vec<char>, comparison_text: Vec<char>) -> Self{
        Frequency::<char> {
            corpus,
            comparison_text
        }
    }

    pub fn fitness(&self, freq_type: Unit, include_space: bool) -> f64{
        let (corpus_freq, text_freq) = Self::get_frequencies(
            &self,
            freq_type,
            include_space
        );

        let mut corpus_vector = Vec::new();
        let mut text_vector = Vec::new();
        
        for ch in 'a'..='z' {
            corpus_vector.push(*corpus_freq.get(&ch).unwrap_or(&0.0));
            text_vector.push(*text_freq.get(&ch).unwrap_or(&0.0));
        }

        if include_space {
            corpus_vector.push(*corpus_freq.get(&' ').unwrap_or(&0.0));
            text_vector.push(*text_freq.get(&' ').unwrap_or(&0.0));
        }

        Self::find_fitness(&corpus_vector, &text_vector)
    }

    fn find_fitness(vec1: &[f64], vec2: &[f64]) -> f64 {
        let dot_product: f64 = vec1
            .iter()
            .zip(vec2) // Pairs each item of vec1 with vec2
            .map(|(x, y)| x * y) // Multiplies each pair
            .sum(); // Sums the product
        let magnitude_vec1: f64 = vec1
            .iter()
            .map(|x| x * x) // Squares each value
            .sum::<f64>() // Sums alls the squared values
            .sqrt(); // Takes the square root of the sum
        let magnitude_vec2: f64 = vec2.iter().map(|x| x * x).sum::<f64>().sqrt();
    
        if magnitude_vec1 == 0.0 || magnitude_vec2 == 0.0 {
            return 0.0; 
        }
        dot_product / (magnitude_vec1 * magnitude_vec2)
    }

    pub fn get_frequencies(&self, freq_type: Unit, include_space: bool) -> (HashMap<char, f64>, HashMap<char, f64>) {
        match freq_type {
            Unit::Char => {
                let comparison_choice = ComparisonChoice::Corpus;
                let corpus_freq = Self::get_char_frequencies(&self, comparison_choice, include_space);
                let comparison_choice = ComparisonChoice::Text;
                let text_freq = Self::get_char_frequencies(&self, comparison_choice, include_space);
                return (corpus_freq.unwrap_or_else(|_| HashMap::new()), text_freq.unwrap_or_else(|_| HashMap::new()));
            },
            Unit::Word => {
                panic!("NOT IMPLEMENTED");
            },
        }
    }

    fn get_char_frequencies(&self, comparison_choice: ComparisonChoice, include_spaces: bool) -> Result<HashMap<char, f64>, std::io::Error> {
        let mut char_counts: HashMap<char, f64> = HashMap::new(); // New hashmap containing characters with their frequency
        let mut total_count = 0.0;
        
        let frequency_text = match comparison_choice {
            ComparisonChoice::Corpus => &self.corpus,
            ComparisonChoice::Text => &self.comparison_text
        };

        for ch in frequency_text {
            let ch_lower: char = ch.to_ascii_lowercase();
            if ch_lower.is_alphabetic() || (include_spaces && ch_lower == ' ') {
                *char_counts.entry(ch_lower).or_insert(0.0) += 1.0;
                total_count += 1.0;
            }
        }
        
        let mut char_freqs = HashMap::new(); // Hashmap containing a percentage of their frequency
        for (ch, count) in char_counts {
            char_freqs.insert(ch, count as f64 / total_count as f64); // Finds what percentage of the text contains this char
        }
     
        Ok(char_freqs)
    }

    // Abandoned
    fn char_to_int(c: char, include_space: bool) -> Option<usize> {
        match c {
            'a'..='z' => {
                Some((c as usize) - ('a' as usize) + 1)
            },
            'A'..='Z' => {
                let this_c = c.to_ascii_lowercase();
                Some((this_c as usize) - ('a' as usize) + 1)
            },
            ' ' => {
                if include_space {
                    Some(27)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}