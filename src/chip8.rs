mod input;
mod instruction;

use self::{
    input::{get_processed_input, Keys},
    instruction::Chip8Instruction,
};
use pixels::Pixels;
use rand::{rngs::ThreadRng, Rng};
use std::fs;
use winit_input_helper::WinitInputHelper;

pub type Screen = [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT];
type Registers = [u8; 16];

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SIZE_4KB: usize = 0x1000;
const FONT: [u8; 5 * 0x10] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pub pixels: Pixels,
    pub input: WinitInputHelper,
    pub screen: Screen,
    memory: [u8; SIZE_4KB],
    pc: usize,
    index: usize,
    stack: Vec<usize>,
    dt: u8,
    st: u8,
    v: Registers,
    keys: Keys,
    rng: ThreadRng,
    paused: bool,
}

impl Chip8 {
    pub fn new(pixels: Pixels) -> Self {
        let mut memory = [0u8; SIZE_4KB];

        // Load font
        memory[0..FONT.len()].copy_from_slice(&FONT);

        Self {
            pixels,
            input: WinitInputHelper::new(),
            memory,
            screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            pc: 0x200,
            index: 0,
            stack: vec![],
            dt: 0,
            st: 0,
            v: [0u8; 16],
            keys: [false; 16],
            rng: rand::thread_rng(),
            paused: false,
        }
    }

    pub fn load_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
        const PROGRAM_START: usize = 0x200;

        let file_data = fs::read(filename)?;

        if file_data.len() > SIZE_4KB - PROGRAM_START {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File is too large to fit into memory.",
            ));
        }

        let program_end = PROGRAM_START + file_data.len();
        self.memory[PROGRAM_START..program_end].copy_from_slice(&file_data);

        println!("File loaded successfully.");
        Ok(())
    }

    pub fn run_cycle(&mut self) {
        if self.paused {
            return;
        }

        let opcode = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc + 1] as u16;
        let instruction = Chip8Instruction::from_opcode(opcode);
        let disassemble = instruction.disassemble();

        println!("[0x{:04X}] => 0x{:04X} | {}", self.pc, opcode, disassemble);

        self.execute_instruction(instruction);
        self.update_timers();
        self.pc += 2;
    }

    pub fn render(&mut self) {
        let frame = self.pixels.frame_mut();
        let screen = &self.screen;
        for (i, frame_pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % SCREEN_WIDTH as usize) as usize;
            let y = (i / SCREEN_WIDTH as usize) as usize;
            let screen_pixel = screen[y][x];

            let rgba = if screen_pixel {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x11, 0x11, 0x11, 0xff]
            };

            frame_pixel.copy_from_slice(&rgba);
        }
    }

    pub fn update_input(&mut self) {
        let (keys, toggle_pause) = get_processed_input(&self.input);
        self.keys = keys;
        if toggle_pause {
            self.paused = !self.paused;
        }
    }

    fn update_timers(&mut self) {
        self.dt -= if self.dt > 0 { 1 } else { 0 };
        self.st -= if self.st > 0 { 1 } else { 0 };
    }

    fn execute_instruction(&mut self, instruction: Chip8Instruction) {
        match instruction {
            Chip8Instruction::SYS(_) => todo!(),
            Chip8Instruction::CLS => self.screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            Chip8Instruction::RET => self.pc = self.stack.pop().unwrap(),
            Chip8Instruction::JP(nnn) => self.pc = nnn - 2,
            Chip8Instruction::CALL(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn - 2;
            }
            Chip8Instruction::SEVxByte(x, kk) => self.pc += if self.v[x] == kk { 2 } else { 0 },
            Chip8Instruction::SNEVxByte(x, kk) => self.pc += if self.v[x] != kk { 2 } else { 0 },
            Chip8Instruction::SEVxVy(x, y) => self.pc += if self.v[x] == self.v[y] { 2 } else { 0 },
            Chip8Instruction::LDVxByte(x, kk) => self.v[x] = kk,
            Chip8Instruction::ADDVxByte(x, kk) => self.v[x] = self.v[x].wrapping_add(kk),
            Chip8Instruction::LDVxVy(x, y) => self.v[x] = self.v[y],
            Chip8Instruction::ORVxVy(x, y) => self.v[x] = self.v[x] | self.v[y],
            Chip8Instruction::ANDVxVy(x, y) => self.v[x] = self.v[x] & self.v[y],
            Chip8Instruction::XORVxVy(x, y) => self.v[x] = self.v[x] ^ self.v[y],
            Chip8Instruction::ADDVxVy(x, y) => {
                let result = self.v[x] as u16 + self.v[y] as u16;
                self.v[0xF] = if result > 255 { 1 } else { 0 };
                self.v[x] = result as u8;
            }
            Chip8Instruction::SUBVxVy(x, y) => {
                self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            }
            Chip8Instruction::SHRVx(x) => {
                self.v[0xF] = self.v[x] & 0b1;
                self.v[x] = self.v[x].wrapping_div(2);
            }
            Chip8Instruction::SUBNVxVy(x, y) => {
                self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            }
            Chip8Instruction::SHLVx(x) => {
                self.v[0xF] = self.v[x] & 0b1;
                self.v[x] = self.v[x].wrapping_mul(2);
            }
            Chip8Instruction::SNEVxVy(x, y) => {
                self.pc += if self.v[x] != self.v[y] { 2 } else { 0 }
            }
            Chip8Instruction::LDI(nnn) => self.index = nnn,
            Chip8Instruction::JP0(nnn) => self.pc = nnn + self.v[0] as usize,
            Chip8Instruction::RNDVxByte(x, kk) => self.v[x] = self.rng.gen_range(0..=255) & kk,
            Chip8Instruction::DRWVxVyNibble(x, y, n) => self.draw_sprite(x, y, n),
            Chip8Instruction::SKPVx(x) => {
                self.pc += if self.keys[self.v[x] as usize] { 2 } else { 0 }
            }
            Chip8Instruction::SKNPVx(x) => {
                self.pc += if !self.keys[self.v[x] as usize] { 2 } else { 0 }
            }
            Chip8Instruction::LDVxDT(x) => self.v[x] = self.dt,
            Chip8Instruction::LDVxK(_) => todo!(),
            Chip8Instruction::LDDTVx(x) => self.dt = self.v[x],
            Chip8Instruction::LDSTVx(x) => self.st = self.v[x],
            Chip8Instruction::ADDIVx(x) => self.index += self.v[x] as usize,
            Chip8Instruction::LDFVx(x) => self.index = self.v[x] as usize * 5,
            Chip8Instruction::LDBVx(x) => {
                let value = self.v[x];
                self.memory[self.index] = value / 100;
                self.memory[self.index + 1] = (value % 100) / 10;
                self.memory[self.index + 2] = value % 10;
            }
            Chip8Instruction::LDIVx(x) => {
                for i in 0..=x as usize {
                    self.memory[self.index + i] = self.v[i];
                }
            }
            Chip8Instruction::LDVxMem(x) => {
                for i in 0..=x as usize {
                    self.v[i] = self.memory[self.index + i];
                }
            }
            Chip8Instruction::Unknown => {}
        };
    }

    fn draw_sprite(&mut self, x: usize, y: usize, n: u8) {
        let mut collision = false;

        for row in 0..n as usize {
            let byte = self.memory[self.index + row];

            for col in 0..8usize {
                if byte & (0x80 >> col) != 0 {
                    let screen_y = (self.v[y] as usize + row).clamp(0, SCREEN_HEIGHT - 1);
                    let screen_x = (self.v[x] as usize + col).clamp(0, SCREEN_WIDTH - 1);
                    let pixel = &mut self.screen[screen_y][screen_x];
                    *pixel ^= true;
                    collision = if *pixel { true } else { collision };
                }
            }
        }

        self.v[0xF] = collision as u8;
    }
}
