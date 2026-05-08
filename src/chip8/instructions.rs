use rand::RngExt;

use crate::chip8::{Chip8, HEIGHT, WIDTH, to_hex};

#[allow(non_snake_case)]
impl Chip8 {
    pub fn execute(&mut self, instr: u16) {
        let nnn: u16 = instr & 0xFFF;
        let nn: u8 = (instr & 0xFF) as u8;
        let n: u8 = (instr & 0xF) as u8;
        let x: usize = (((instr & 0xF00) >> 8) as u8) as usize;
        let y: usize = (((instr & 0xF0) >> 4) as u8) as usize; 

        match instr & 0xF000 {
            0x0000 => {
                match nnn {
                    0xE0 => self.display = [0; WIDTH*HEIGHT],               // disp_clear()
                    0xEE => {                                               // RET
                        if let Some(val) = self.stack.pop() {
                            self.pc = val;
                            self.sp -= 1;
                        } 
                    },
                    _ => unreachable!("0x0000 shouldn't have instr {}!", to_hex(instr))
                }
            },
            0x1000 => self.pc = nnn,                                        // pc = nnn
            0x2000 => {                                                     // Call subroutine
                self.stack.push(self.pc);
                self.sp += 1;
                self.pc = nnn;
            },
            0x3000 => if self.registers[x] == nn { self.pc += 2 },          // if vx == nn, skip next instr
            0x4000 => if self.registers[x] != nn { self.pc += 2 },          // if vx != nn, skip next instr
            0x5000 => 
                if self.registers[x] == self.registers[y] {                 // if vx == vy, skip next instr
                    self.pc += 2 
                },
            0x6000 => self.registers[x] = nn,                               // vx = nn
            0x7000 => self.registers[x] = self.registers[x].wrapping_add(nn),// vx += nn
            0x8000 => self.case_0x8000(x, y, n),
            0x9000 => 
                if self.registers[x] != self.registers[y] {                 // if vx != vy, skip next instr
                    self.pc += 2 
                },
            0xA000 => self.I = nnn,                                         // I = nnn
            0xB000 => self.pc = self.registers[0] as u16 + nnn,             // pc = nnn + v0
            0xC000 => 
                self.registers[x] = rand::rng().random::<u8>() & nn,        // vx = rand() & nn
            0xD000 => self.draw(x, y, n as usize),                  // draw(vx, vy, n)
            0xE000 => {
                let key: usize = self.registers[x] as usize;
                match nn {
                    0x9E => if self.keypad[key] {                      // if key() == vx, skip next instr
                        self.pc += 2 
                    }
                    0xA1 => if !self.keypad[key] {                      // if key() != vx, skip next instr
                        self.pc += 2 
                    }
                    _ => unreachable!("0xE000 shouldn't have instr {}!", to_hex(instr))
                }
            },
            0xF000 => self.case_0xF000(x, nn),
            _ => unreachable!()
        }
    }

    fn case_0x8000(&mut self, x: usize, y: usize, n: u8) {
        match n {
            0x0 => self.registers[x] = self.registers[y],                   // vx = vy
            0x1 => {                                                        // vx |= vy
                self.registers[x] |= self.registers[y];
                self.registers[0xF] = 0;
            },
            0x2 => {                                                        // vx &= vy
                self.registers[x] &= self.registers[y];
                self.registers[0xF] = 0;
            },
            0x3 => {                                                        // vx ^= vy
                self.registers[x] ^= self.registers[y];
                self.registers[0xF] = 0;
            },
            0x4 => {                                                        // vx += vy, set flag if overflow
                let (val, is_overflow) = 
                    self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = val;
                self.registers[0xF] = is_overflow as u8;
            },
            0x5 => {                                                        // vx -= vy, set flag if NOT underflow
                let (val, is_underflow) = 
                    self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = val;
                self.registers[0xF] = (!is_underflow) as u8;
            },
            0x6 => {                                                        // vx >>= 1, set flag to val of least
                // todo: quirks                                             // sig bit before shift is set
                self.registers[x] = self.registers[y];
                let flag: u8 = self.registers[x] & 0x1;
                self.registers[x] >>= 1;
                self.registers[0xF] = flag;
            },
            0x7 => {                                                        // vx = vy - vx, set flag if NOT underflow
                let (val, is_underflow) = 
                    self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = val;
                self.registers[0xF] = (!is_underflow) as u8;
            },
            0xE => {                                                        // vx <<= 1, set flag to val of most
                // todo: quirks                                             // sig bit before shift is set
                self.registers[x] = self.registers[y];
                let flag: u8 = self.registers[x] >> 7;
                self.registers[x] <<= 1;
                self.registers[0xF] = flag;
            },
            _ => unreachable!("0x800N shouldn't have N {}!", to_hex(n))
        }
    }

    fn case_0xF000(&mut self, x: usize, nn: u8) {
        match nn {
            0x07 => self.registers[x] = self.delay_timer,                   // vx = get_delay()
            0x0A => self.get_key(x),                                        // vx = getkey()
            0x15 => self.delay_timer = self.registers[x],                   // delay_timer(vx)
            0x18 => self.sound_timer = self.registers[x],                   // sound_timer(vx)
            0x1E => self.I += self.registers[x] as u16,                     // I += vx
            0x29 => self.I = 5 * self.registers[x] as u16,                  // I = location of sprite for digit vx
            0x33 => {                                                       // set_BCD(vx)
                self.memory[self.I as usize] = self.registers[x] /100;
                self.memory[self.I as usize +1] = (self.registers[x]/10) %10;
                self.memory[self.I as usize +2] = self.registers[x] %10
            },
            0x55 => for offset in 0..=x {                            // reg_dump(vx, &I) 
                self.memory[self.I as usize + offset] = self.registers[offset]
            },
            0x65 => for offset in 0..=x {                           // reg_load(vx, &I)
                self.registers[offset] = self.memory[self.I as usize + offset]
            },
            _ => unreachable!("0x80NN shouldn't have NN {}!", to_hex(nn))
        }
    }
    
    fn draw(&mut self, x: usize, y: usize, height: usize) {
        self.registers[0xF] = 0;
        let x_origin = self.registers[x] as usize & WIDTH - 1;
        let y_origin = self.registers[y] as usize & HEIGHT - 1;
        let (mut x_pos, mut y_pos);

        for h in 0..height {
            y_pos = y_origin + h;
            if y_pos >= HEIGHT { break }

            let sprite_byte = self.memory[self.I as usize + h];

            for b in 0..8 {
                x_pos = x_origin + b;
                if x_pos >= WIDTH { break }

                if (sprite_byte >> (7 - b)) & 0x1 == 0 { continue }

                let index = y_pos * WIDTH + x_pos;

                if self.display[index] == 1 {
                    self.registers[0xF] = 1;
                }
                self.display[index] ^= 1;
            }
        }
    }

    fn get_key(&mut self, x: usize) {
        if !self.waiting_for_key {
            if self.keypad.iter().any(|&k| k) {
                self.waiting_for_key = true;
                self.released_key = x;
            }
            self.pc -= 2;
        } else {
            if let Some(key) = self.keypad.iter().position(|&k| !k) {
                self.registers[self.released_key] = key as u8;
                self.waiting_for_key = false;
            } else {
                self.pc -= 2;
            }
        }
    }

}