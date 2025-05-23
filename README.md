# clamshell

> A quick and dirty to-do file viewer and manipulator. 

## Installation and Build

Clone the repository and navigate into it:

```bash
git clone https://github.com/DriftingOtter/clamshell.git
cd clamshell
```

### Required: Set the To-Do File Path

Specify the location of your to-do file using the `TODO_FILE_PATH` environment variable:

```bash
export TODO_FILE_PATH=/home/USERNAME_HERE/Documents/todo.org
```

### Optional: Configure Default Editor and Viewer

You can optionally set which programs are used for editing and viewing the to-do file:

```bash
export DEFAULT_EDITOR=vim   # Default is nano
export DEFAULT_VIEWER=less  # Default is cat
```

These values will be compiled into the binary. You can still override them at runtime using CLI flags.

### Build the Project

```bash
cargo build --release
```

### Optional: Move Binary to Local Binaries Folder

To access `clamshell` without specifying the binary path, move it to your local binaries folder:

```bash
cp target/release/clamshell ~/.local/bin
```

Ensure `~/.local/bin` is in your `PATH`. You can add it by updating your shell configuration file (`~/.bashrc` or `~/.zshrc`):

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

### Modes

- **View mode**: `clamshell`
- **Edit mode**: `clamshell -e` or `clamshell --edit`
- **Help mode**: `clamshell -h` or `clamshell --help`

### Additional Arguments

- `--editor <name>`: Override the default editor (e.g., vim, nano).
- `--viewer <name>`: Override the default viewer (e.g., cat, less).

## Authors

- Daksh Kaul // DriftingOtter 🦦

