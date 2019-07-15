type Address = u16;
type Register = u8;
type Data = u8;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    ClearScreen,                              // 00E0 - CLS
    Return,                                   // 00EE - RET
    SYS,                                      // 0nnn - SYS addr
    Jump(Address),                            // 1nnn - JP addr
    Call(Address),                            // 2nnn - CALL addr
    SkipEqualsByte(Register, Data),           // 3xkk - SE vx, byte
    SkipNotEqualsByte(Register, Data),        // 4xkk - SNE Vx, byte
    SkipEqualsRegister(Register, Register),   // 5xy0 - SE Vx, Vy
    LoadByte(Register, Data),                 // 6xkk - LD Vx, byte
    AddByte(Register, Data),                  // 7xkk - ADD Vx, byte
    LoadRegister(Register, Register),         // 8xy0 - LD Vx, Vy
    Or(Register, Register),                   // 8xy1 - OR Vx, Vy
    And(Register, Register),                  // 8xy2 - AND Vx, Vy
    Xor(Register, Register),                  // 8xy3 - XOR Vx, Vy
    AddRegister(Register, Register),          // 8xy4 - ADD Vx, Vy
    SubRegister(Register, Register),          // 8xy5 - SUB Vx, Vy
    ShiftRight(Register),                     // 8xy6 - SHR Vx
    SubNRegister(Register, Register),         // 8xy7 - SUBN Vx, Vy
    ShiftLeft(Register),                      // 8xyE - SHL Vx
    SkipNotEqualRegister(Register, Register), // 9xy0 - SNE Vx, Vy
    LoadImmediate(Address),                   // Annn - LD I, addr
    JumpBase(Address),                        // Bnnn - JP V0, address
    Random(Register, Data),                   // Cxkk - RND Vx, byte
    DisplaySprite(Register, Register, u8),    // Dxyn - DRW Vx, Vy, nibble
    SkipKeyPress(Register),                   // Ex9E - SKP Vx
    SkipNotKeyPress(Register),                // ExA1 - SKNP Vx
    LoadFromDelay(Register),                  // Fx07 - LD Vx, DT
    LoadKeyPress(Register),                   // Fx0A - LD Vx, K
    LoadDelay(Register),                      // Fx15 - LD DT, Vx
    LoadSound(Register),                      // Fx18 - LD ST, Vx
    AddI(Register),                           // Fx1E - ADD I, Vx
    LoadFontSprite(Register),                 // Fx29 - LD F, Vx
    LoadIBCD(Register),                       // Fx33 - LD B, Vx
    StoreRegisters(Register),                 // Fx55 - LD [I], Vx
    LoadRegisters(Register),                  // Fx65 - LD Vx, [I]
}

pub trait InstructionParser {
    fn try_from(&self, opcode: u16) -> Result<Instruction, String>;
}
