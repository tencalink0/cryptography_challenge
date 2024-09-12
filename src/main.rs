mod utils;

use utils::{get_user_input};
use std::path::Path;
use std::env;

pub fn read_file(file_name: String) -> String {
    let file_path = Path::new(
                        &env::current_dir()
                            .expect("Failed to get current directory")
                            .parent()
                            .expect("Failed to get parent directory"))
        .join("assets")
        .join("dummy_text.txt");

    println!("FILE PATH: {}", file_path.clone().display());
    let file_contents = std::fs::read_to_string(file_path.clone())
        .expect(&format!("Error reading {}", file_path.clone().display()));
    
    return file_contents;
}

fn main() {
    println!("Choose your challenge: (1,8)");
    let input = get_user_input();
    println!("{}", input);

    match input.as_str() {
        "1" => {
            let contents = read_file("dummy_text.txt".to_string());
            println!("{}", contents);
        }
        _ => {
            println!("Invalid choice");
        }
    }
}
