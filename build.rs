use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // Check for TODO_FILE_PATH environment variable
    // If not set, use the default file path
    let todo_file_path = env::var("TODO_FILE_PATH")
        .unwrap_or_else(|_| { "todo.org".to_string()});

    // Retrieve directory where Cargo places build artifacts
    let output_directory = env::var("OUT_DIR")
        .expect("Failed to retrieve OUT_DIR");

    // Set path for generated file_path.rs
    let destination_path = Path::new(&output_directory).join("file_path.rs");

    // Create file_path.rs
    let mut file = File::create(&destination_path)
        .expect("Failed to create file_path.rs file.");

    // Write the TODO_FILE_PATH as a constant
    write!(file, "pub const TODO_FILE_PATH: &str = {:?};", todo_file_path)
        .expect("Failed to write to file_path.rs file.");
}

