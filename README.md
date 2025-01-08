# clamshell

> A quick and dirty to-do file viewer and manipulator. 

## Installation and Build

Clone the repository and navigate into it:
```bash
git clone https://github.com/DriftingOtter/clamshell.git
cd clamshell
```

Set the environment variable for your to-do file and build the project:
```bash
export TODO_FILE_PATH=/home/USERNAME_HERE/Documents/todo.org
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

## Synopsis

- **View mode**: `clamshell`
- **Edit mode**: `clamshell -e` or `clamshell --edit`
- **Help mode**: `clamshell -h` or `clamshell --help`

### Additional Arguments

- `--editor <name>`: Specifies the editor to use (e.g., vim, nano). Defaults to `nano`.
- `--viewer <name>`: Specifies the viewer to use (e.g., cat, less). Defaults to `cat`.

## Authors

- Daksh Kaul // DriftingOtter 🦦

