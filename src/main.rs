use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Machine {
    name: String,
    counter: u16,
    stack_ptr: u8,
    mem: [u8; 4096],
    stack: [u16; 16],
    v: [u8; 16],        // registers: v0 to vf
    i: u16,             // "There is also a 16-bit register called I."
    delay_register: u8,
    sound_register: u8,
}

impl Machine {

    fn new(name: &str) -> Machine {
        let mem = [0; 4096];
        let stack = [0; 16];
        let registers = [0; 16];
        let chip8 = Machine {
            name: String::from("Chip8"),
            counter: 0,
            stack_ptr: 0,
            mem,
            stack,
            v: registers,
            i: 0,
            delay_register: 0,
            sound_register: 0
        };
        return chip8;
    }

    fn copy_rom(&mut self) -> [u8;4096] {
        // TODO: Read the filename from program arguments
        let filename = "/Users/manishwingify/Personaldev/Rust/chip8/roms/pong.rom";
        let mut file = File::open(filename).expect("ROM not found");

        let bufsize = 4096 - 512;
        let mut buffer: [u8; bufsize] = [0; bufsize];

        // load the ROM into the buffer
        file.read(&mut buffer);

        // Copy the buffer into the VM memory
        // TODO: Why not copy directly without the intermediate buffer
        self.mem[512..].clone_from_slice(&buffer);
        return self.mem;
    }
}

fn main() {
    let mut vm = Machine::new("Chip8");
}
