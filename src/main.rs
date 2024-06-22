use std::env;
use std::process::Command;

// Include generated file path defined in TODO_FILE_PATH environment constant
include!(concat!(env!("OUT_DIR"), "/file_path.rs"));

fn get_command_line_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Too many command-line arguments, check -h for manual page.");
    } else if args.len() == 1 {
        return "".to_string();
    }

    return args[1].clone();
}

fn edit_mode() {
    let status = Command::new("nvim")
        .arg(TODO_FILE_PATH)
        .status()
        .expect("Failed to execute edit command.");

    if !status.success() {
        eprintln!("nvim exited with an error.");
    }
}

fn view_mode() {
    let status = Command::new("bat")
        .arg(TODO_FILE_PATH)
        .status()
        .expect("Failed to execute edit command.");

    if !status.success() {
        eprintln!("bat exited with an error.");
    }
}

fn help_mode() {
    let help_page: &str = r#"
    NAME
            todo

    SYNOPSIS

            View mode: todo
            Edit mode: todo -e
            Man  mode: todo -h

    DESCRIPTION
                
            A command-line tool to help view, edit, and use your to-do files.

    OPTIONS

            -e
                Opens to-do file in neovim.

            -h
                Outputs this help message.

    AUTHOR
                
            Daksh Kaul // DriftingOtter
    "#;

    println!("{}", help_page);
}

fn main() {
    match get_command_line_args().as_str() {
        "-e" => edit_mode(),
        "-h" => help_mode(),
        ""   => view_mode(),
        _    => eprintln!("Invalid command-line argument. Please use -h to open manual page."),
    };
}

