# Rust Chip 8 Emulator

The classic emulation learning starting point, made in Rust & Raylib

### How to build

Simply have Rust & use `cargo run` or use `cargo build` and use the binary in `target/debug/`, either named `chip8` or `chip8.exe`

### How to use

Run the program in a locatio with a `games` folder, that includes `.ch8` files. Using Up/Down or W/S will allow you to choose a game from that folder. Press Space to enter the game. 
The games controls will vary, but below I will note the Chip-8 bindings. Press 'Escape' to exit

### Chip8 Control bindings

Typically followed a format of

1 | 2 | 3 | C\
4 | 5 | 6 | D\
7 | 8 | 9 | E\
A | 0 | B | F

The current layout on a keyboard is maps to:

1 | 2 | 3 | 4\
q | w | e | r\
a | s | d | f\
z | x | c | v
