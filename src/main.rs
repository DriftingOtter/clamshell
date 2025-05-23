use std::env;
use std::process::{Command, exit};

// path for the to-do file, defined at build time <-- build.rs
include!(concat!(env!("OUT_DIR"), "/file_path.rs"));

/// Parses command-line arguments and returns:
/// mode(s): "edit", "help", or "" for view
/// - editor: optionally specified text editor
/// - viewer: optionally specified viewer
fn get_command_line_args() -> (&'static str, Option<String>, Option<String>) {
    let args: Vec<String> = env::args().collect();

    let mut mode = "";
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

/// Executes the given command (editor or viewer) on the TODO file.
/// Returns the exit status of the command.
fn exec_command(command_name: &str) -> i32 {
    let status = Command::new(command_name)
        .arg(TODO_FILE_PATH)
        .status()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute `{}` command.", command_name);
            exit(1);
        });

    if !status.success() {
        let code = status.code().unwrap_or(1);
        eprintln!("`{}` exited with error code {}.", command_name, code);
        return code;
    }

    return status.code().unwrap_or(0);
}

/// Prints the help manual to the console.
fn help_mode() {
    const HELP_PAGE: &str = r#"
NAME
    clamshell

SYNOPSIS
    View mode: clamshell
    Edit mode: clamshell --edit  -e
    Man  mode: clamshell --help  -h

DESCRIPTION
    A command-line tool to help view, edit, and use your to-do files.

OPTIONS
    --edit
        Opens to-do file in the specified editor (default: nano).

    --help
        Outputs this help message.

    --editor <name>
        Specifies the editor to use (e.g., vim, nano).

    --viewer <name>
        Specifies the viewer to use (e.g., cat, less).

AUTHOR
    Daksh Kaul // DriftingOtter
"#;
    println!("{}", HELP_PAGE);
}

fn main() {
    let (mode, editor, viewer) = get_command_line_args();

    // Set defaults only if not specified
    let editor = editor.unwrap_or_else(|| "nano".to_string());
    let viewer = viewer.unwrap_or_else(|| "cat".to_string());

    match mode {
        "edit" => { exit(exec_command(&editor)); },
        "help" => help_mode(),
        ""     => { exit(exec_command(&viewer)); },
        _      => {
            eprintln!("Invalid argument. Use -h or --help for usage.");
            exit(1);
        }
    };
}

