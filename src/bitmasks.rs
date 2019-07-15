#[allow(non_snake_case)]
#[allow(clippy::cast_possible_truncation)]
pub const fn mask_F000(opcode: u16) -> u8 {
    ((opcode >> 12) & 0xF) as u8
}

#[allow(non_snake_case)]
#[allow(clippy::cast_possible_truncation)]
pub const fn mask_00FF(opcode: u16) -> u8 {
    (opcode & 0xFF) as u8
}

#[allow(non_snake_case)]
#[allow(clippy::cast_possible_truncation)]
pub const fn mask_0F00(opcode: u16) -> u8 {
    ((opcode >> 8) & 0xF) as u8
}

#[allow(non_snake_case)]
pub const fn mask_0FFF(opcode: u16) -> u16 {
    opcode & 0xFFF
}

#[allow(non_snake_case)]
#[allow(clippy::cast_possible_truncation)]
pub const fn mask_00F0(opcode: u16) -> u8 {
    ((opcode >> 4) & 0xF) as u8
}

#[allow(non_snake_case)]
#[allow(clippy::cast_possible_truncation)]
pub const fn mask_000F(opcode: u16) -> u8 {
    (opcode & 0xF) as u8
}
