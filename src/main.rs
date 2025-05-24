use std::env;
use std::process::{Command, exit};
use std::fs;

// Imports compile-time constants
include!(concat!(env!("OUT_DIR"), "/file_path.rs"));

/// Parses command-line arguments and returns:
/// mode(s): "edit", "help", or "" (view)
/// - optional editor override
/// - optional viewer override
fn get_command_line_args() -> (&'static str, Option<String>, Option<String>) {
    let args: Vec<String> = env::args().collect();

    let mut mode   = "";
    let mut editor = None;
    let mut viewer = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-e" | "--edit" => mode = "edit",
            "-h" | "--help" => mode = "help",
            "--editor" => {
                if i + 1 < args.len() {
                    editor = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for --editor <name>.");
                    exit(1);
                }
            },
            "--viewer" => {
                if i + 1 < args.len() {
                    viewer = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for --viewer <name>.");
                    exit(1);
                }
            },
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                exit(1);
            },
        }
        i += 1;
    }

    return (mode, editor, viewer);
}

/// Executes the given command with the todo file as an argument.
/// Returns the exit code of the process.
fn exec_command(command_name: &str) -> i32 {
    let status = Command::new(command_name)
        .arg(TODO_FILE_PATH)
        .status()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute `{}`.", command_name);
            exit(1);
        });

    if !status.success() {
        let code = status.code().unwrap_or(1);
        eprintln!("{} exited with an error code {}.", command_name, code);
        return code;
    }

    return status.code().unwrap_or(0);
}

/// Displays the man page
fn help_mode() {
    println!(
        r#"
NAME
    clamshell

SYNOPSIS
    View mode: clamshell
    Edit mode: clamshell --edit  -e
    Man  mode: clamshell --help  -h

DESCRIPTION
    A quick and dirty to-do file viewer and manipulator.

OPTIONS
    --edit
        Opens to-do file in the specified editor (default: {editor}).

    --help
        Outputs this help message.

    --editor <name>
        Specifies the editor to use (e.g., vim, nano).

    --viewer <name>
        Specifies the viewer to use (e.g., cat, less).

AUTHOR
    Daksh Kaul // DriftingOtter
"#,
        editor = DEFAULT_EDITOR
    );
}

fn main() {
    let (mode, editor_arg, viewer_arg) = get_command_line_args();

    let editor = editor_arg.unwrap_or_else(|| DEFAULT_EDITOR.to_string());
    let viewer = viewer_arg.unwrap_or_else(|| DEFAULT_VIEWER.to_string());

    match mode {
        "edit" => {
            exec_command(&editor);
        }
        "help" => help_mode(),
        "" => {
            if viewer == "cat" {
                match fs::read_to_string(TODO_FILE_PATH) {
                    Ok(contents) => println!("{}", contents),
                    Err(err) => {
                        eprintln!("Error reading file: {}", err);
                        exit(1);
                    }
                }
            } else {
                exec_command(&viewer);
            }
        }
        _ => {
            eprintln!("Invalid mode. Use -h or --help for usage.");
            exit(1);
        }
    }
}

