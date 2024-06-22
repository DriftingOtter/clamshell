# clamshell

> A quick and dirty to-do file viewer, and manipulator. 

## Installation // Build 

```zsh
$ git clone https://github.com/DriftingOtter/clamshell.git
$ cd clamshell
```

```zsh
$ export TODO_FILE_PATH=/home/USERNAME_HERE/Documents/todo.org
$ cargo build --release
```

### **__OPTIONAL__** Build Step

```zsh
$ cp /target/release/clamshell ~/.local/bin 
```
## SYNOPSIS

- View mode: clamshell
- Edit mode: clamshell -e
- Man  mode: clamshell -h

## Dependencies

- Neovim (for editor, but can be changed in code)
- bat    (for outputting todo file contents, but can be changed in code)

## Authors

- Daksh Kaul // DrifitingOtter 🦦
