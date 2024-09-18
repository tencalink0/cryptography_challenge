use std::env;

pub fn get_user_input() -> String {
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    let _ = stdout().flush();

    stdin()
        .read_line(&mut input)
        .expect("An error occured");

    //clean string
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}

pub fn read_file(file_name: String) -> String {
    let file_path = env::current_dir()
        .expect("Failed to get current directory")
        .join("assets")
        .join(file_name);

    println!("FILE PATH: {}", file_path.clone().display());
    let file_contents = std::fs::read_to_string(file_path.clone())
            .expect(&format!("Error reading {}", file_path.clone().display()));
    
        
    file_contents
}
