struct Chip8 {
    ram: [u8; 4096],
    stack: [u16; 16],
    v: [u8; 16], // registers
    i: u16, // I register
    timers: [u16; 2], // Delay and Sound timers
    pc: u16, // program counter
    sp: u8, // stack pointer
    buf: [bool; 64*32], // display buffer
}

fn main() {
    println!("Hello, world!");
}
