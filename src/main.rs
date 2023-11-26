use std::env;
use std::fs;

fn main() {
    // Find all files in the arguments sent to the program
    let passed_args: Vec<String> = env::args().collect();

    // Whenever a file is found, add the file contents to the string (resulting string)
    let paths = &passed_args[1..];

    let mut result = String::new();

    // Loop through each path and find the file
    for path in paths.iter() {
        match fs::read_to_string(path) {
            Ok(res) => result.push_str(&res),
            Err(err) => result.push_str(&err.to_string()),
        }

        // result.push_str("\r\n\r\n");
    }

    // Otherwise, if the file is not found, we continue to the next file and log an error for that specific file
    println!("{}", result);
}
