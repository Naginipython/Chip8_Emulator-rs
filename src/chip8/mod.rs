use std::{error::Error, fmt::UpperHex, fs};

mod instructions;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const START_ADDR: u16 = 0x200;
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

// bool is_released;

#[allow(non_snake_case)]
pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 0xFFF],
    I: u16,
    pc: u16,
    stack: Vec<u16>,
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 0x10],
    display: [u8; WIDTH*HEIGHT],

    // others
    waiting_for_key: bool,
    released_key: usize,
}

impl Chip8 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn load_rom(&mut self, file: &str) -> Result<(), Box<dyn Error>> {
        let bytes: Vec<u8> = fs::read(file)?;

        for (i, byte) in bytes.iter().enumerate() {
            self.memory[START_ADDR as usize + i] = *byte;
        }

        Ok(())
    }
    pub fn step(&mut self) {
        let opcode: u16 = u16::from_be_bytes([self.memory[self.pc as usize], self.memory[self.pc as usize + 1]]);
        println!("Instr: {}", to_hex(opcode));

        self.pc += 2;

        self.execute(opcode);
    }
    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
    pub fn get_display(&self) -> &[u8; WIDTH*HEIGHT] {
        &self.display
    }
    pub fn toggle_key(&mut self, key_idx: usize, state: bool) {
        if key_idx >= self.keypad.len() { return; }
        self.keypad[key_idx] = state;
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut memory = [0; 0xFFF];

        for (i, b) in FONTSET.iter().enumerate() {
            memory[0x50 + i] = *b;
        }

        Self {
            registers: [0; 16],
            memory,
            I: 0,
            pc: START_ADDR,
            stack: Vec::new(),
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 0x10],
            display: [0; WIDTH*HEIGHT],

            waiting_for_key: false,
            released_key: 0
        }
    }
}

pub fn to_hex<T: UpperHex>(byte: T) -> String {
    format!("{:#06X}", byte)
}
