use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Read environment variables or use defaults
    let todo_file_path = env::var("TODO_FILE_PATH").unwrap_or_else(|_| "todo.md".into());
    let default_editor = env::var("DEFAULT_EDITOR").unwrap_or_else(|_| "nano".into());
    let default_viewer = env::var("DEFAULT_VIEWER").unwrap_or_else(|_| "cat".into());

    // Determine path for generated Rust file
    let out_dir = env::var("OUT_DIR").expect("Failed to read OUT_DIR");
    let dest_path = Path::new(&out_dir).join("file_path.rs");

    // Write all constants to the file
    let content = format!(
        r#"pub const TODO_FILE_PATH: &str = {todo:?};
pub const DEFAULT_EDITOR: &str = {editor:?};
pub const DEFAULT_VIEWER: &str = {viewer:?};
"#,
        todo   = todo_file_path,
        editor = default_editor,
        viewer = default_viewer,
    );

    fs::write(dest_path, content).expect("Failed to write to file_path.rs");
}

