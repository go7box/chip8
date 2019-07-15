use crate::bitmasks::*;
use crate::instructions::{Instruction, InstructionParser};

#[allow(dead_code)]
pub struct OpcodeMaskParser {}

impl InstructionParser for OpcodeMaskParser {
    fn try_from(&self, opcode: u16) -> Result<Instruction, String> {
        match mask_F000(opcode) {
            0x0 => match mask_00FF(opcode) {
                0xE0 => Ok(Instruction::ClearScreen),
                0xEE => Ok(Instruction::Return),
                _ => Ok(Instruction::SYS),
            },
            0x1 => Ok(Instruction::Jump(mask_0FFF(opcode))),
            0x2 => Ok(Instruction::Call(mask_0FFF(opcode))),
            0x3 => Ok(Instruction::SkipEqualsByte(
                mask_0F00(opcode),
                mask_00FF(opcode),
            )),
            0x4 => Ok(Instruction::SkipNotEqualsByte(
                mask_0F00(opcode),
                mask_00FF(opcode),
            )),
            0x5 => Ok(Instruction::SkipEqualsRegister(
                mask_0F00(opcode),
                mask_00F0(opcode),
            )),
            0x6 => Ok(Instruction::LoadByte(mask_0F00(opcode), mask_00FF(opcode))),
            0x7 => Ok(Instruction::AddByte(mask_0F00(opcode), mask_00FF(opcode))),
            0x8 => {
                let r1 = mask_0F00(opcode);
                let r2 = mask_00F0(opcode);
                match mask_000F(opcode) {
                    0x0 => Ok(Instruction::LoadRegister(r1, r2)),
                    0x1 => Ok(Instruction::Or(r1, r2)),
                    0x2 => Ok(Instruction::And(r1, r2)),
                    0x3 => Ok(Instruction::Xor(r1, r2)),
                    0x4 => Ok(Instruction::AddRegister(r1, r2)),
                    0x5 => Ok(Instruction::SubRegister(r1, r2)),
                    0x6 => Ok(Instruction::ShiftRight(r1)),
                    0x7 => Ok(Instruction::SubNRegister(r1, r2)),
                    0xE => Ok(Instruction::ShiftLeft(r1)),
                    _ => Err(format!("Invalid opcode: {:X}", opcode)),
                }
            }
            0x9 => Ok(Instruction::SkipNotEqualRegister(
                mask_0F00(opcode),
                mask_00F0(opcode),
            )),
            0xA => Ok(Instruction::LoadImmediate(mask_0FFF(opcode))),
            0xB => Ok(Instruction::JumpBase(mask_0FFF(opcode))),
            0xC => Ok(Instruction::Random(mask_0F00(opcode), mask_00FF(opcode))),
            0xD => Ok(Instruction::DisplaySprite(
                mask_0F00(opcode),
                mask_00F0(opcode),
                mask_000F(opcode),
            )),
            0xE => {
                let register = mask_0F00(opcode);
                match mask_00FF(opcode) {
                    0x9E => Ok(Instruction::SkipKeyPress(register)),
                    0xA1 => Ok(Instruction::SkipNotKeyPress(register)),
                    _ => Err(format!("Invalid opcode: {:X}", opcode)),
                }
            }
            0xF => {
                let register = mask_0F00(opcode);
                match mask_00FF(opcode) {
                    0x07 => Ok(Instruction::LoadFromDelay(register)),
                    0x0A => Ok(Instruction::LoadKeyPress(register)),
                    0x15 => Ok(Instruction::LoadDelay(register)),
                    0x18 => Ok(Instruction::LoadSound(register)),
                    0x1E => Ok(Instruction::AddI(register)),
                    0x29 => Ok(Instruction::LoadFontSprite(register)),
                    0x33 => Ok(Instruction::LoadIBCD(register)),
                    0x55 => Ok(Instruction::StoreRegisters(register)),
                    0x65 => Ok(Instruction::LoadRegisters(register)),
                    _ => Err(format!("Invalid opcode: {:X}", opcode)),
                }
            }
            _ => Err(format!("opcode {:X} is invalid", opcode)),
        }
    }
}
