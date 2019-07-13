use std::fmt;
use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGISTER_COUNT: usize = 16;
const PROGRAM_OFFSET: usize = 512;

struct Memory {
    mem: [u8; MEMORY_SIZE],
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const ZERO: u8 = 0;
        write!(f, "{{ ")?;
        for (index, byte) in self.mem.iter().enumerate() {
            if *byte != ZERO {
                write!(f, "{}: {}, ", index, byte)?;
            }
        }
        write!(f, "}}")
    }
}

pub struct Machine {
    name: String,
    counter: u16,
    stack_ptr: u8,
    mem: Memory,
    stack: [u16; STACK_SIZE],
    v: [u8; REGISTER_COUNT], // registers: v0 to vf
    i: u16,                  // "There is also a 16-bit register called I."
    delay_register: u8,
    sound_register: u8,
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {{ \n\tPC: {}, \n\tSP: {}, \n\tStack: {:?}, \n\tRegisters: {:?}, \n\ti: {}, \n\tDR: {}, \n\tSR: {}, \n\tMem: {:?} }}", self.name, self.counter, self.stack_ptr, self.stack, self.v, self.i, self.delay_register, self.sound_register, self.mem)
    }
}

impl Machine {
    pub fn new(name: &str) -> Self {
        Machine {
            name: name.to_string(),
            counter: 0,
            stack_ptr: 0,
            mem: Memory {
                mem: [0; MEMORY_SIZE],
            },
            stack: [0; STACK_SIZE],
            v: [0; REGISTER_COUNT],
            i: 0,
            delay_register: 0,
            sound_register: 0,
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = File::open(filename)?;
        self._copy_into_mem(&mut file)
    }

    fn _copy_into_mem(&mut self, file: &mut File) -> Result<(), std::io::Error> {
        const BUFSIZE: usize = MEMORY_SIZE - PROGRAM_OFFSET;
        let mut buffer: [u8; BUFSIZE] = [0; BUFSIZE];

        // load the ROM into the buffer
        let _ = file.read(&mut buffer)?;

        // Copy the buffer into the VM memory
        // TODO: Why not copy directly without the intermediate buffer
        self.mem.mem[PROGRAM_OFFSET..].clone_from_slice(&buffer);
        Ok(())
    }

    // Start the virtual machine: This is the fun part!
    pub fn start(&mut self) -> Result<(), std::io::Error> {
        loop {
            // we check for 4095 because we need to read 2 bytes.
            if self.counter > 4095 {
                panic!("Program Counter Out of bounds!");
            }
            let (fb, sb) = (
                self.mem.mem[self.counter as usize],
                self.mem.mem[(self.counter + 1) as usize],
            );
            let opcode = self.create_opcode(fb, sb);
            println!("Opcode = {}", opcode);
        }
        Ok(())
    }

    /**
    * Create a 16-bit opcode out of 2 bytes
    * Ref: https://stackoverflow.com/a/50244328
    * Shift the bits by 8 to the left:
        (XXXXXXXX becomes 00000000XXXXXXXX)
    * THEN bitwise-OR to concatenate them:
    *   (00000000XXXXXXXX | YYYYYYYY) = XXXXXXXXYYYYYYYY
    **/
    fn create_opcode(&mut self, fb: u8, sb: u8) -> u16 {
        let mut fb_u16 = u16::from(fb);
        let sb_u16 = u16::from(sb);
        fb_u16 <<= 8;
        fb_u16 | sb_u16
    }
}

#[cfg(test)]
use std::io::{Seek, SeekFrom, Write};
mod tests {

    #[test]
    fn test_copy_into_mem_no_data() {
        let mut tmpfile = tempfile::tempfile().unwrap();
        let mut vm = Machine::new("TestVM");
        vm._copy_into_mem(&mut tmpfile).unwrap();
        assert_eq!(vm.mem.mem.len(), 4096);
        // every byte in memory is zero when file is empty
        for byte in vm.mem.mem.iter() {
            assert_eq!(*byte, 0);
        }
    }

    #[test]
    fn test_copy_into_mem_some_data() {
        let mut tmpfile = tempfile::tempfile().unwrap();
        let mut vm = Machine::new("TestVM");
        write!(tmpfile, "Hello World!").unwrap(); // Write
        tmpfile.seek(SeekFrom::Start(0)).unwrap(); // Seek to start
        vm._copy_into_mem(&mut tmpfile).unwrap();
        let expected = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33];
        let mut count = 0;
        for _ in 0..expected.len() {
            assert_eq!(vm.mem.mem[PROGRAM_OFFSET + count], expected[count]);
            count += 1;
        }
    }

    #[test]
    fn test_create_opcode() {
        let mut vm = Machine::new("TestVM");
        assert_eq!(vm.create_opcode(0x31, 0x42), 0x3142);
        assert_eq!(vm.create_opcode(0x1, 0x2), 0x0102);
        assert_eq!(vm.create_opcode(0xAB, 0x9C), 0xAB9C);

        // doesn't magically append or prepend zeroes to the final output
        assert_ne!(vm.create_opcode(0x1, 0x2), 0x1200);
        assert_ne!(vm.create_opcode(0x1, 0x2), 0x0012);
    }
}
