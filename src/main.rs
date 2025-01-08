use std::env;
use std::process::{Command, exit};

// Include generated file path defined in TODO_FILE_PATH environment constant
include!(concat!(env!("OUT_DIR"), "/file_path.rs"));

fn get_command_line_args() -> (String, Option<String>, Option<String>) {
    let args: Vec<String> = env::args().collect();
    let mut mode = "".to_string();
    let mut editor = None;
    let mut viewer = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-e" | "--edit" => mode = "edit".to_string(),
            "-h" | "--help" => mode = "help".to_string(),
            "--editor" => {
                if i + 1 < args.len() {
                    editor = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for --editor <name>.");
                    exit(-1);
                }
            },
            "--viewer" => {
                if i + 1 < args.len() {
                    viewer = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for --viewer <name>.");
                    exit(-1);
                }
            },
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                exit(-1);
            },
        }
        i += 1;
    }

    return (mode, editor, viewer);
}

fn exec_command(command_name: &str) -> i32 {
    let status = Command::new(command_name)
        .arg(TODO_FILE_PATH)
        .status()
        .expect(format!("Failed to execute {} command.", command_name).as_str());

    if !status.success() {
        let exit_code = match status.code() {
            Some(code) => code,
            None => -1,
        };
        eprintln!("{} exited with an error {}.", command_name, exit_code);
        return exit_code;
    }

    return status.code().unwrap_or(0);
}

fn help_mode() {
    let help_page: &str = r#"
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

    println!("{}", help_page);
}

fn main() {
    let (mode, editor, viewer) = get_command_line_args();

    let editor = editor.unwrap_or_else(|| "nano".to_string());
    let viewer = viewer.unwrap_or_else(|| "cat".to_string());

    match mode.as_str() {
        "edit" => {
            exec_command(&editor);
        },
        "help" => help_mode(),
        ""   => {
            exec_command(&viewer);
        },
        _    => {
            eprintln!("Invalid command-line argument. Please use -h or --help to open manual page.");
            exit(-1);
        },
    };
}

