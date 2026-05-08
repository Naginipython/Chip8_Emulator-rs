use std::{fs, path::Path, process::exit};

use raylib::{RaylibHandle, RaylibThread, color::Color, drawing::RaylibDraw, ffi::KeyboardKey};

use crate::chip8::{Chip8, HEIGHT, WIDTH};

mod chip8;

const SCALE: i32 = 10;
const STEPS_PER_FRAME: u32 = 12;

fn main() {
    let mut chip8 = Chip8::new();

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32 * SCALE, HEIGHT as i32 * SCALE)
        .title("Chip-8 Emulator")
        .build();

    rl.set_target_fps(60);

    let game = choose_game_menu(&mut rl, &thread);

    let game_path = format!("games/{}", &game[2..]);
    if let Err(e) = chip8.load_rom(&game_path.trim()) {
        eprintln!("{e}");
        exit(1);
    }

    play_game(rl, thread, chip8);
}

// ########## CHOOSING GAME TO PLAY ##########

fn choose_game_menu(rl: &mut RaylibHandle, thread: &RaylibThread) -> String {
    let mut game_idx: i32 = 0;
    let mut known_game_len = 0;
    let mut game_chosen = String::new();
    let mut games: Vec<String> = vec![];
    while !rl.window_should_close() {
        // handle menu input
        if rl.is_key_pressed(KeyboardKey::KEY_W) || rl.is_key_pressed(KeyboardKey::KEY_UP) {
            if game_idx - 1 < 0 {
                game_idx = known_game_len - 1;
            } else {
                game_idx -= 1;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_S) || rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            if game_idx + 1 > known_game_len - 1 {
                game_idx = 0;
            } else {
                game_idx += 1;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game_chosen = games[game_idx as usize].clone();
            break;
        }

        // handle display
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        // Handle no dir issue
        if !Path::new("games").is_dir() {
            let text = "ERROR: No games directory found!";
            let text_width = d.measure_text(text, 24);
            d.draw_text(
                text,
                (WIDTH as i32 * SCALE - text_width) / 2,
                (HEIGHT as i32 * SCALE - 24) / 2,
                24,
                Color::WHITE
            );
            continue;
        }

        // handle finding games
        games = fs::read_dir("games")
            .expect("Failed to read directory")
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "ch8" {
                    Some(path.file_name()?.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect();

        if games.len() == 0 {
            let text = "ERROR: No games found in games directory!";
            let text_width = d.measure_text(text, 24);
            d.draw_text(
                text,
                (WIDTH as i32 * SCALE - text_width) / 2,
                (HEIGHT as i32 * SCALE - 24) / 2,
                24,
                Color::WHITE
            );
        }

        known_game_len = games.len() as i32;

        // handle listing games
        if games.len() * 24 < HEIGHT * SCALE as usize {
            games[game_idx as usize] = format!("> {}  ", games[game_idx as usize]);
            let text = games.join("\n");
            let text_width = d.measure_text(&text, 24);
            d.draw_text(
                &text,
                (WIDTH as i32 * SCALE - text_width) / 2,
                (HEIGHT as i32 * SCALE - 24*games.len() as i32) / 2,
                24,
                Color::WHITE
            );
        } else {
            todo!()
        }
    }

    game_chosen
}


// ########## PLAYING THE GAME ##########

fn play_game(mut rl: RaylibHandle, thread: RaylibThread, mut chip8: Chip8) {
    while !rl.window_should_close() {
        handle_game_key(&rl, &mut chip8);

        for _ in 0..STEPS_PER_FRAME {
            chip8.step();
        }
        chip8.tick_timers();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if chip8.get_display()[y * WIDTH + x] != 0 {
                    d.draw_rectangle(
                        x as i32 * SCALE,
                        y as i32 * SCALE,
                        SCALE,
                        SCALE,
                        Color::WHITE
                    );
                }
            }
        }
    }
}

fn handle_game_key(rl: &RaylibHandle, chip8: &mut Chip8) {
    chip8.toggle_key(0x1, rl.is_key_down(KeyboardKey::KEY_ONE));
    chip8.toggle_key(0x2, rl.is_key_down(KeyboardKey::KEY_TWO));
    chip8.toggle_key(0x3, rl.is_key_down(KeyboardKey::KEY_THREE));
    chip8.toggle_key(0xC, rl.is_key_down(KeyboardKey::KEY_FOUR));
    chip8.toggle_key(0x4, rl.is_key_down(KeyboardKey::KEY_Q));
    chip8.toggle_key(0x5, rl.is_key_down(KeyboardKey::KEY_W));
    chip8.toggle_key(0x6, rl.is_key_down(KeyboardKey::KEY_E));
    chip8.toggle_key(0xD, rl.is_key_down(KeyboardKey::KEY_R));
    chip8.toggle_key(0x7, rl.is_key_down(KeyboardKey::KEY_A));
    chip8.toggle_key(0x8, rl.is_key_down(KeyboardKey::KEY_S));
    chip8.toggle_key(0x9, rl.is_key_down(KeyboardKey::KEY_D));
    chip8.toggle_key(0xE, rl.is_key_down(KeyboardKey::KEY_F));
    chip8.toggle_key(0xA, rl.is_key_down(KeyboardKey::KEY_Z));
    chip8.toggle_key(0x0, rl.is_key_down(KeyboardKey::KEY_X));
    chip8.toggle_key(0xB, rl.is_key_down(KeyboardKey::KEY_C));
    chip8.toggle_key(0xF, rl.is_key_down(KeyboardKey::KEY_V));
}
