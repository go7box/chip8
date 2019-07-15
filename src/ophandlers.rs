use crate::bitmasks::{mask_000F, mask_00F0, mask_00FF, mask_0F00, mask_0FFF};
use crate::instructions::Instruction;

#[allow(non_snake_case)]
pub const fn handle0x00E0(_opcode: u16) -> Instruction {
    Instruction::ClearScreen
}

#[allow(non_snake_case)]
pub const fn handle0x00EE(_opcode: u16) -> Instruction {
    Instruction::Return
}

#[allow(non_snake_case)]
pub const fn handle0x0NNN(_opcode: u16) -> Instruction {
    Instruction::SYS
}

#[allow(non_snake_case)]
pub const fn handle0x1NNN(opcode: u16) -> Instruction {
    Instruction::Jump(mask_0FFF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0x2NNN(opcode: u16) -> Instruction {
    Instruction::Call(mask_0FFF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0x3XNN(opcode: u16) -> Instruction {
    Instruction::SkipEqualsByte(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0x4XNN(opcode: u16) -> Instruction {
    Instruction::SkipNotEqualsByte(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0x5XY0(opcode: u16) -> Instruction {
    Instruction::SkipEqualsRegister(mask_0F00(opcode), mask_00F0(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0x6XNN(opcode: u16) -> Instruction {
    Instruction::LoadByte(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0x7XNN(opcode: u16) -> Instruction {
    Instruction::AddByte(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0x8XY0(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    Instruction::LoadRegister(r1, r2)
}

#[allow(non_snake_case)]
pub const fn handle0x8XY1(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    Instruction::Or(r1, r2)
}

#[allow(non_snake_case)]
pub const fn handle0x8XY2(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    Instruction::And(r1, r2)
}

#[allow(non_snake_case)]
pub const fn handle0x8XY3(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    Instruction::Xor(r1, r2)
}

#[allow(non_snake_case)]
pub const fn handle0x8XY4(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    Instruction::AddRegister(r1, r2)
}

#[allow(non_snake_case)]
pub const fn handle0x8XY5(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    Instruction::SubRegister(r1, r2)
}

#[allow(non_snake_case)]
pub const fn handle0x8XY6(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    Instruction::ShiftRight(r1)
}

#[allow(non_snake_case)]
pub const fn handle0x8XY7(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    Instruction::SubNRegister(r1, r2)
}

#[allow(non_snake_case)]
pub const fn handle0x8XYE(opcode: u16) -> Instruction {
    let r1 = mask_0F00(opcode);
    Instruction::ShiftLeft(r1)
}

#[allow(non_snake_case)]
pub const fn handle0x9XY0(opcode: u16) -> Instruction {
    Instruction::SkipNotEqualRegister(mask_0F00(opcode), mask_00F0(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0xANNN(opcode: u16) -> Instruction {
    Instruction::LoadImmediate(mask_0FFF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0xBNNN(opcode: u16) -> Instruction {
    Instruction::JumpBase(mask_0FFF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0xCXNN(opcode: u16) -> Instruction {
    Instruction::Random(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0xDXYN(opcode: u16) -> Instruction {
    Instruction::DisplaySprite(mask_0F00(opcode), mask_00F0(opcode), mask_000F(opcode))
}

#[allow(non_snake_case)]
pub const fn handle0xEX9E(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::SkipKeyPress(register)
}

#[allow(non_snake_case)]
pub const fn handle0xEXA1(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::SkipNotKeyPress(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX07(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::LoadFromDelay(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX0A(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::LoadKeyPress(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX15(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::LoadDelay(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX18(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::LoadSound(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX1E(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::AddI(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX29(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::LoadFontSprite(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX33(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::LoadIBCD(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX55(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::StoreRegisters(register)
}

#[allow(non_snake_case)]
pub const fn handle0xFX65(opcode: u16) -> Instruction {
    let register = mask_0F00(opcode);
    Instruction::LoadRegisters(register)
}
