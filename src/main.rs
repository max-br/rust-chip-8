const RAM_SIZE: usize = 4096;
const PROGRAM_OFFSET: usize = 64; // 0x200 / 8

const COLS: usize = 64;
const ROWS: usize = 32;

use std::fs::File;
use std::io::Read;
use std::string::String;

struct Chip8 {
    ram: [u8; RAM_SIZE],
    stack: [usize; 16],
    v: [u8; 16], // registers
    i: u16, // I register
    timers: [u16; 2], // Delay and Sound timers
    pc: usize, // program counter
    sp: usize, // stack pointer
    buf: [bool; ROWS*COLS], // display buffer
}

impl Chip8 {
    fn new(game_path: String) -> Chip8 {
        let mut chip = Chip8 {
            ram: [0; RAM_SIZE],
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            timers: [0, 2],
            pc: PROGRAM_OFFSET,
            sp: 0,
            buf: [false; ROWS*COLS],
        };
        chip.init_ram(game_path);
        chip
    }

    fn init_ram(&mut self, game_path: String) {
        let mut file = File::open(game_path).unwrap();
        file.read(&mut self.ram[PROGRAM_OFFSET ..]).unwrap();
    }

    fn cpu_step(&mut self) {
        let address_bits = |h, l| -> u16 { (((h & 0x0F) as u16) << 8) | l as u16 };

        let (h, l) = (self.ram[self.pc], self.ram[self.pc+1]);
        let (hh, hl) = (h & 0xF0, h & 0x0F);
        let (lh, ll) = (l & 0xF0, l & 0x0F);
        // match opcode
        match hh {
            0x0 => match l {
                0xE0 => self.buf = [false; ROWS*COLS], // CLS
                0xEE => { // RET
                    self.pc = self.stack[self.sp];
                    self.sp -= 1;
                },
                _ => panic!("Invalid Opcode 1"),
            },
            0x1 => self.pc = address_bits(h,l) as usize, // JP addr
            0x2 => { // CALL addr
                self.sp += 1;
                self.stack[self.sp] = self.pc;
                self.pc = address_bits(h, l) as usize;
            },
            0x3 => if self.v[hl as usize] == l { self.pc += 2 }, // SE
            0x4 => if self.v[hl as usize] != l { self.pc += 2 }, // SNE
            0x5 => if self.v[hl as usize] == self.v[lh as usize] { self.pc += 2 }, // SE
            0x6 => self.v[hl as usize] = l, // LD
            0x7 => self.v[hl as usize] += l, // ADD
            0x8 => match ll {
                0x0 => println!("LD"),
                0x1 => println!("OR"),
                0x2 => println!("AND"),
                0x3 => println!("XOR"),
                0x4 => println!("ADD"),
                0x5 => println!("SUB"),
                0x6 => println!("SHR"),
                0x7 => println!("SUBN"),
                0xE => println!("SHL"),
                _ => panic!("Invalid Opcode"),
            },
            0x9 => println!("SNE"),
            0xA => println!("LD"),
            0xB => println!("JP"),
            0xC => println!("RND"),
            0xD => println!("DRW"),
            0xE => println!("SKP, SKNP"),
            0xF => println!("LD, LD, LD, LD, ADD, LD, LD, LD, LD"),
            _ => panic!("Invalid Opcode"),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let game_path = "/home/max/Downloads/c8games/TETRIS".to_string();
    let mut chip = Chip8::new(game_path);

}
