use std::any::type_name;
use std::any::Any;

pub enum Unit {
    Char,
    Word,
}

fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

#[derive(Debug, Clone)]
pub struct Frequency<T> {
    vec: Vec<T>,
}

impl<T: Clone + Any + 'static> Frequency<T> { // Impl trait must have the derive methods included
    pub fn new(vec: Vec<T>) -> Self{
        Frequency::<T> {
            vec,
        }
    }

    pub fn get_frequencies(&self, freq_type: Unit, include_space: bool) -> Vec<(usize, usize)> {
        match freq_type {
            Unit::Char => {
                if include_space {
                    return Vec::from(Self::get_char_frequencies::<27>(&self));
                } else {
                    return Vec::from(Self::get_char_frequencies::<26>(&self));
                }
            },
            Unit::Word => {
                panic!("NOT IMPLEMENTED");
            },
        }
    }

    pub fn get_char_frequencies<const F_SIZE: usize>(&self) -> [(usize, usize); F_SIZE] {
        let mut frequencies: [(usize, usize); F_SIZE] = 
            (1..=F_SIZE) // Declares an iterator with the numbers 1 to 26 (or 27)
                .map(|num| (num, 0)) // Converts to an iterator and to the desired form
                .collect::<Vec<_>>() // Collects the iterator and converts to a vector
                .try_into() // Tries to convert to an array
                .expect("Incorrect Length"); // Error call
        
        for c in self.vec.clone() {
            if let Some(c_char) = 
                (&c as &dyn Any) // Coercion to convert c to Any type (to prepare being casted as char)
                    .downcast_ref::<char>() { // Cast T to a char
                let char_num = 
                    if F_SIZE == 27 { 
                        Self::char_to_int(*c_char, true) 
                    } 
                    else { 
                        Self::char_to_int(*c_char, false) 
                };

                match char_num {
                    Some(num) => {
                        frequencies[num-1].1 += 1;
                    },
                    None => {},
                }
            } else {
                println!("Warning: T is not a char!");
            }
        }
        frequencies
    }

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