use super::sm80::Core;
use crate::cpu::registers::Flag;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Operand {
    pub byte: u8,
    pub word: u16,
}

pub struct Instruction {
    pub name: &'static str,
    pub opcode: u8,
    pub operand_length: u8,
    pub cycles: u8,
    pub exec: Box<dyn Fn(&mut Core, Option<Operand>)>,
}

impl Instruction {
    pub fn new(
        name: &'static str,
        opcode: u8,
        operand_length: u8,
        cycles: u8,
        exec: Box<dyn Fn(&mut Core, Option<Operand>)>,
    ) -> Instruction {
        Instruction {
            name,
            opcode,
            operand_length,
            cycles,
            exec,
        }
    }
}

pub struct InstructionSet {
    instructions: HashMap<u8, Instruction>,
    cb_instructions: HashMap<u8, Instruction>,
}

impl InstructionSet {
    pub fn new() -> InstructionSet {
        let (instructions, cb_instructions) = get_instruction_set();
        Self {
            instructions,
            cb_instructions,
        }
    }
    pub fn get_next_executable_instruction(
        &self,
        core: &mut Core,
    ) -> Option<(&Instruction, Option<Operand>)> {
        let opcode = core.get_next();
        let mut instruction = self.instructions.get(&opcode);
        if opcode == 0xcb {
            instruction = self.cb_instructions.get(&core.get_next());
        }

        instruction.map(|instruction| match instruction.operand_length {
            0 => (instruction, None),
            1 => (
                instruction,
                Some(Operand {
                    byte: core.get_next(),
                    word: 0x00,
                }),
            ),
            2 => (
                instruction,
                Some(Operand {
                    word: core.get_next_word(),
                    byte: 0x00,
                }),
            ),
            _ => panic!("Invalid operand length of {}", instruction.operand_length),
        })
    }
}

pub fn get_instruction_set() -> (HashMap<u8, Instruction>, HashMap<u8, Instruction>) {
    let mut instruction_set: HashMap<u8, Instruction> = HashMap::new();
    let mut cb_instruction_set: HashMap<u8, Instruction> = HashMap::new();

    instruction_set.insert(0x00, Instruction::new("nop", 0x00, 0, 4, Box::new(nop)));
    instruction_set.insert(
        0x01,
        Instruction::new("ld bc, d16", 0x01, 2, 12, Box::new(load_bc_d16)),
    );
    instruction_set.insert(
        0x02,
        Instruction::new("ld (bc), a", 0x02, 0, 8, Box::new(load_mem_bc_a)),
    );
    instruction_set.insert(
        0x03,
        Instruction::new("inc bc", 0x03, 0, 8, Box::new(increment_bc)),
    );
    instruction_set.insert(
        0x04,
        Instruction::new("inc b", 0x04, 0, 4, Box::new(increment_b)),
    );
    instruction_set.insert(
        0x05,
        Instruction::new("dec b", 0x05, 0, 4, Box::new(decrement_b)),
    );
    instruction_set.insert(
        0x06,
        Instruction::new("ld b, d8", 0x06, 1, 8, Box::new(load_b_d8)),
    );
    instruction_set.insert(
        0x07,
        Instruction::new("rlca", 0x07, 0, 4, Box::new(rotate_left_carry_a)),
    );
    instruction_set.insert(
        0x08,
        Instruction::new("ld (a16), sp", 0x08, 2, 20, Box::new(load_mem_sp)),
    );
    instruction_set.insert(
        0x09,
        Instruction::new("add hl, bc", 0x09, 0, 8, Box::new(add_hl_bc)),
    );
    instruction_set.insert(
        0x0a,
        Instruction::new("ld a, (bc)", 0x0a, 0, 8, Box::new(load_a_mem_bc)),
    );
    instruction_set.insert(
        0x0b,
        Instruction::new("dec bc", 0x0b, 0, 8, Box::new(decrement_bc)),
    );
    instruction_set.insert(
        0x0c,
        Instruction::new("inc c", 0x0c, 0, 4, Box::new(increment_c)),
    );
    instruction_set.insert(
        0x0d,
        Instruction::new("dec c", 0x0d, 0, 4, Box::new(decrement_c)),
    );
    instruction_set.insert(
        0x0e,
        Instruction::new("ld c, d8", 0x0e, 1, 8, Box::new(load_c_d8)),
    );
    instruction_set.insert(
        0x0f,
        Instruction::new("rrca", 0x0f, 0, 4, Box::new(rotate_right_carry_a)),
    );
    instruction_set.insert(0x10, Instruction::new("stop", 0x10, 1, 4, Box::new(stop)));
    instruction_set.insert(
        0x11,
        Instruction::new("ld de, d16", 0x11, 2, 12, Box::new(load_de_d16)),
    );
    instruction_set.insert(
        0x12,
        Instruction::new("ld (de), a", 0x12, 0, 8, Box::new(load_mem_de_a)),
    );
    instruction_set.insert(
        0x13,
        Instruction::new("inc de", 0x13, 0, 8, Box::new(increment_de)),
    );
    instruction_set.insert(
        0x14,
        Instruction::new("inc d", 0x14, 0, 4, Box::new(increment_d)),
    );
    instruction_set.insert(
        0x15,
        Instruction::new("dec d", 0x15, 0, 4, Box::new(decrement_d)),
    );
    instruction_set.insert(
        0x16,
        Instruction::new("ld d, d8", 0x16, 1, 8, Box::new(load_d_d8)),
    );
    instruction_set.insert(
        0x17,
        Instruction::new("rla", 0x17, 0, 4, Box::new(rotate_left_a_through)),
    );
    instruction_set.insert(0x18, Instruction::new("jr r8", 0x18, 1, 8, Box::new(jr_r8)));
    instruction_set.insert(
        0x19,
        Instruction::new("add hl, de", 0x19, 0, 8, Box::new(add_hl_de)),
    );
    instruction_set.insert(
        0x1a,
        Instruction::new("ld a, (de)", 0x1a, 0, 8, Box::new(load_a_mem_de)),
    );
    instruction_set.insert(
        0x1b,
        Instruction::new("dec de", 0x1b, 0, 8, Box::new(decrement_de)),
    );
    instruction_set.insert(
        0x1c,
        Instruction::new("inc e", 0x1c, 0, 4, Box::new(increment_e)),
    );
    instruction_set.insert(
        0x1d,
        Instruction::new("dec e", 0x1d, 0, 4, Box::new(decrement_e)),
    );
    instruction_set.insert(
        0x1e,
        Instruction::new("ld e, d8", 0x1e, 1, 8, Box::new(load_e_d8)),
    );
    instruction_set.insert(
        0x1f,
        Instruction::new("rra", 0x1f, 0, 4, Box::new(rotate_right_a_through)),
    );
    instruction_set.insert(
        0x20,
        Instruction::new("jr nz, r8", 0x20, 1, 8, Box::new(jr_nz_r8)),
    );
    instruction_set.insert(
        0x21,
        Instruction::new("ld hl, d16", 0x21, 2, 12, Box::new(load_hl_d16)),
    );
    instruction_set.insert(
        0x22,
        Instruction::new("ld (hl+), a", 0x22, 0, 8, Box::new(load_mem_hlp_a)),
    );
    instruction_set.insert(
        0x23,
        Instruction::new("inc hl", 0x23, 0, 8, Box::new(increment_hl)),
    );
    instruction_set.insert(
        0x24,
        Instruction::new("inc h", 0x24, 0, 4, Box::new(increment_h)),
    );
    instruction_set.insert(
        0x25,
        Instruction::new("dec h", 0x25, 0, 4, Box::new(decrement_h)),
    );
    instruction_set.insert(
        0x26,
        Instruction::new("ld h, d8", 0x26, 1, 8, Box::new(load_h_d8)),
    );
    instruction_set.insert(
        0x27,
        Instruction::new("daa", 0x27, 0, 4, Box::new(decimal_adjust_a)),
    );
    instruction_set.insert(
        0x28,
        Instruction::new("jr z, r8", 0x28, 1, 8, Box::new(jr_z_r8)),
    );
    instruction_set.insert(
        0x29,
        Instruction::new("add hl, hl", 0x29, 0, 8, Box::new(add_hl_hl)),
    );
    instruction_set.insert(
        0x2a,
        Instruction::new("ld a, (hl+)", 0x2a, 0, 8, Box::new(load_a_hlp_mem)),
    );
    instruction_set.insert(
        0x2b,
        Instruction::new("dec hl", 0x2b, 0, 8, Box::new(decrement_hl)),
    );
    instruction_set.insert(
        0x2c,
        Instruction::new("inc l", 0x2c, 0, 4, Box::new(increment_l)),
    );
    instruction_set.insert(
        0x2d,
        Instruction::new("dec l", 0x2d, 0, 4, Box::new(decrement_l)),
    );
    instruction_set.insert(
        0x2e,
        Instruction::new("ld l, d8", 0x2e, 1, 8, Box::new(load_l_d8)),
    );
    instruction_set.insert(
        0x2f,
        Instruction::new("cpl", 0x2f, 0, 4, Box::new(complement_a)),
    );
    instruction_set.insert(
        0x30,
        Instruction::new("jr nc, r8", 0x30, 1, 8, Box::new(jr_nc_r8)),
    );
    instruction_set.insert(
        0x31,
        Instruction::new("ld sp, d16", 0x31, 2, 12, Box::new(load_sp_d16)),
    );
    instruction_set.insert(
        0x32,
        Instruction::new("ld (hl-), a", 0x32, 0, 8, Box::new(load_mem_hlm_a)),
    );
    instruction_set.insert(
        0x33,
        Instruction::new("inc sp", 0x33, 0, 8, Box::new(increment_sp)),
    );
    instruction_set.insert(
        0x34,
        Instruction::new("inc (hl)", 0x34, 0, 12, Box::new(increment_mem_hl)),
    );
    instruction_set.insert(
        0x35,
        Instruction::new("dec (hl)", 0x35, 0, 12, Box::new(decrement_mem_hl)),
    );
    instruction_set.insert(
        0x36,
        Instruction::new("ld (hl), d8", 0x36, 1, 12, Box::new(load_mem_hl_d8)),
    );
    instruction_set.insert(
        0x37,
        Instruction::new("scf", 0x37, 0, 4, Box::new(set_carry)),
    );
    instruction_set.insert(
        0x38,
        Instruction::new("jr c, r8", 0x38, 1, 8, Box::new(jr_c_r8)),
    );
    instruction_set.insert(
        0x39,
        Instruction::new("add hl, sp", 0x39, 0, 8, Box::new(add_hl_sp)),
    );
    instruction_set.insert(
        0x3a,
        Instruction::new("ld a, (hl-)", 0x3a, 0, 8, Box::new(load_a_hlm_mem)),
    );
    instruction_set.insert(
        0x3b,
        Instruction::new("dec sp", 0x3b, 0, 8, Box::new(decrement_sp)),
    );
    instruction_set.insert(
        0x3c,
        Instruction::new("inc a", 0x3c, 0, 4, Box::new(increment_a)),
    );
    instruction_set.insert(
        0x3d,
        Instruction::new("dec a", 0x3d, 0, 4, Box::new(decrement_a)),
    );
    instruction_set.insert(
        0x3e,
        Instruction::new("ld a, d8", 0x3e, 1, 8, Box::new(load_a_d8)),
    );
    instruction_set.insert(
        0x3f,
        Instruction::new("ccf", 0x3f, 0, 4, Box::new(complement_carry)),
    );
    instruction_set.insert(
        0x40,
        Instruction::new("ld b, b", 0x40, 0, 4, Box::new(load_b_b)),
    );
    instruction_set.insert(
        0x41,
        Instruction::new("ld b, c", 0x41, 0, 4, Box::new(load_b_c)),
    );
    instruction_set.insert(
        0x42,
        Instruction::new("ld b, d", 0x42, 0, 4, Box::new(load_b_d)),
    );
    instruction_set.insert(
        0x43,
        Instruction::new("ld b, e", 0x43, 0, 4, Box::new(load_b_e)),
    );
    instruction_set.insert(
        0x44,
        Instruction::new("ld b, h", 0x44, 0, 4, Box::new(load_b_h)),
    );
    instruction_set.insert(
        0x45,
        Instruction::new("ld b, l", 0x45, 0, 4, Box::new(load_b_l)),
    );
    instruction_set.insert(
        0x46,
        Instruction::new("ld b, (hl)", 0x46, 0, 8, Box::new(load_b_mem_hl)),
    );
    instruction_set.insert(
        0x47,
        Instruction::new("ld b, a", 0x47, 0, 4, Box::new(load_b_a)),
    );
    instruction_set.insert(
        0x48,
        Instruction::new("ld c, b", 0x48, 0, 4, Box::new(load_c_b)),
    );
    instruction_set.insert(
        0x49,
        Instruction::new("ld c, c", 0x49, 0, 4, Box::new(load_c_c)),
    );
    instruction_set.insert(
        0x4a,
        Instruction::new("ld c, d", 0x4a, 0, 4, Box::new(load_c_d)),
    );
    instruction_set.insert(
        0x4b,
        Instruction::new("ld c, e", 0x4b, 0, 4, Box::new(load_c_e)),
    );
    instruction_set.insert(
        0x4c,
        Instruction::new("ld c, h", 0x4c, 0, 4, Box::new(load_c_h)),
    );
    instruction_set.insert(
        0x4d,
        Instruction::new("ld c, l", 0x4d, 0, 4, Box::new(load_c_l)),
    );
    instruction_set.insert(
        0x4e,
        Instruction::new("ld c, (hl)", 0x4e, 0, 8, Box::new(load_c_mem_hl)),
    );
    instruction_set.insert(
        0x4f,
        Instruction::new("ld c, a", 0x4f, 0, 4, Box::new(load_c_a)),
    );
    instruction_set.insert(
        0x50,
        Instruction::new("ld d, b", 0x50, 0, 4, Box::new(load_d_b)),
    );
    instruction_set.insert(
        0x51,
        Instruction::new("ld d, c", 0x51, 0, 4, Box::new(load_d_c)),
    );
    instruction_set.insert(
        0x52,
        Instruction::new("ld d, d", 0x52, 0, 4, Box::new(load_d_d)),
    );
    instruction_set.insert(
        0x53,
        Instruction::new("ld d, e", 0x53, 0, 4, Box::new(load_d_e)),
    );
    instruction_set.insert(
        0x54,
        Instruction::new("ld d, h", 0x54, 0, 4, Box::new(load_d_h)),
    );
    instruction_set.insert(
        0x55,
        Instruction::new("ld d, l", 0x55, 0, 4, Box::new(load_d_l)),
    );
    instruction_set.insert(
        0x56,
        Instruction::new("ld d, (hl)", 0x56, 0, 8, Box::new(load_d_mem_hl)),
    );
    instruction_set.insert(
        0x57,
        Instruction::new("ld d, a", 0x57, 0, 4, Box::new(load_d_a)),
    );
    instruction_set.insert(
        0x58,
        Instruction::new("ld e, b", 0x58, 0, 4, Box::new(load_e_b)),
    );
    instruction_set.insert(
        0x59,
        Instruction::new("ld e, c", 0x59, 0, 4, Box::new(load_e_c)),
    );
    instruction_set.insert(
        0x5a,
        Instruction::new("ld e, d", 0x5a, 0, 4, Box::new(load_e_d)),
    );
    instruction_set.insert(
        0x5b,
        Instruction::new("ld e, e", 0x5b, 0, 4, Box::new(load_e_e)),
    );
    instruction_set.insert(
        0x5c,
        Instruction::new("ld e, h", 0x5c, 0, 4, Box::new(load_e_h)),
    );
    instruction_set.insert(
        0x5d,
        Instruction::new("ld e, l", 0x5d, 0, 4, Box::new(load_e_l)),
    );
    instruction_set.insert(
        0x5e,
        Instruction::new("ld e, (hl)", 0x5e, 0, 8, Box::new(load_e_mem_hl)),
    );
    instruction_set.insert(
        0x5f,
        Instruction::new("ld e, a", 0x5f, 0, 4, Box::new(load_e_a)),
    );
    instruction_set.insert(
        0x60,
        Instruction::new("ld h, b", 0x60, 0, 4, Box::new(load_h_b)),
    );
    instruction_set.insert(
        0x61,
        Instruction::new("ld h, c", 0x61, 0, 4, Box::new(load_h_c)),
    );
    instruction_set.insert(
        0x62,
        Instruction::new("ld h, d", 0x62, 0, 4, Box::new(load_h_d)),
    );
    instruction_set.insert(
        0x63,
        Instruction::new("ld h, e", 0x63, 0, 4, Box::new(load_h_e)),
    );
    instruction_set.insert(
        0x64,
        Instruction::new("ld h, h", 0x64, 0, 4, Box::new(load_h_h)),
    );
    instruction_set.insert(
        0x65,
        Instruction::new("ld h, l", 0x65, 0, 4, Box::new(load_h_l)),
    );
    instruction_set.insert(
        0x66,
        Instruction::new("ld h, (hl)", 0x66, 0, 8, Box::new(load_h_mem_hl)),
    );
    instruction_set.insert(
        0x67,
        Instruction::new("ld h, a", 0x67, 0, 4, Box::new(load_h_a)),
    );
    instruction_set.insert(
        0x68,
        Instruction::new("ld l, b", 0x68, 0, 4, Box::new(load_l_b)),
    );
    instruction_set.insert(
        0x69,
        Instruction::new("ld l, c", 0x69, 0, 4, Box::new(load_l_c)),
    );
    instruction_set.insert(
        0x6a,
        Instruction::new("ld l, d", 0x6a, 0, 4, Box::new(load_l_d)),
    );
    instruction_set.insert(
        0x6b,
        Instruction::new("ld l, e", 0x6b, 0, 4, Box::new(load_l_e)),
    );
    instruction_set.insert(
        0x6c,
        Instruction::new("ld l, h", 0x6c, 0, 4, Box::new(load_l_h)),
    );
    instruction_set.insert(
        0x6d,
        Instruction::new("ld l, l", 0x6d, 0, 4, Box::new(load_l_l)),
    );
    instruction_set.insert(
        0x6e,
        Instruction::new("ld l, (hl)", 0x6e, 0, 8, Box::new(load_l_mem_hl)),
    );
    instruction_set.insert(
        0x6f,
        Instruction::new("ld l, a", 0x6f, 0, 4, Box::new(load_l_a)),
    );
    instruction_set.insert(
        0x70,
        Instruction::new("ld (hl), b", 0x70, 0, 8, Box::new(load_mem_hl_b)),
    );
    instruction_set.insert(
        0x71,
        Instruction::new("ld (hl), c", 0x71, 0, 8, Box::new(load_mem_hl_c)),
    );
    instruction_set.insert(
        0x72,
        Instruction::new("ld (hl), d", 0x72, 0, 8, Box::new(load_mem_hl_d)),
    );
    instruction_set.insert(
        0x73,
        Instruction::new("ld (hl), e", 0x73, 0, 8, Box::new(load_mem_hl_e)),
    );
    instruction_set.insert(
        0x74,
        Instruction::new("ld (hl), h", 0x74, 0, 8, Box::new(load_mem_hl_h)),
    );
    instruction_set.insert(
        0x75,
        Instruction::new("ld (hl), l", 0x75, 0, 8, Box::new(load_mem_hl_l)),
    );
    instruction_set.insert(0x76, Instruction::new("halt", 0x76, 0, 4, Box::new(halt)));
    instruction_set.insert(
        0x77,
        Instruction::new("ld (hl), a", 0x77, 0, 8, Box::new(load_mem_hl_a)),
    );
    instruction_set.insert(
        0x78,
        Instruction::new("ld a, b", 0x78, 0, 4, Box::new(load_a_b)),
    );
    instruction_set.insert(
        0x79,
        Instruction::new("ld a, c", 0x79, 0, 4, Box::new(load_a_c)),
    );
    instruction_set.insert(
        0x7a,
        Instruction::new("ld a, d", 0x7a, 0, 4, Box::new(load_a_d)),
    );
    instruction_set.insert(
        0x7b,
        Instruction::new("ld a, e", 0x7b, 0, 4, Box::new(load_a_e)),
    );
    instruction_set.insert(
        0x7c,
        Instruction::new("ld a, h", 0x7c, 0, 4, Box::new(load_a_h)),
    );
    instruction_set.insert(
        0x7d,
        Instruction::new("ld a, l", 0x7d, 0, 4, Box::new(load_a_l)),
    );
    instruction_set.insert(
        0x7e,
        Instruction::new("ld a, (hl)", 0x6e, 0, 8, Box::new(load_a_mem_hl)),
    );
    instruction_set.insert(
        0x7f,
        Instruction::new("ld a, a", 0x7f, 0, 4, Box::new(load_a_a)),
    );
    instruction_set.insert(
        0x80,
        Instruction::new("add a, b", 0x80, 0, 4, Box::new(add_b)),
    );
    instruction_set.insert(
        0x81,
        Instruction::new("add a, c", 0x81, 0, 4, Box::new(add_c)),
    );
    instruction_set.insert(
        0x82,
        Instruction::new("add a, d", 0x82, 0, 4, Box::new(add_d)),
    );
    instruction_set.insert(
        0x83,
        Instruction::new("add a, e", 0x83, 0, 4, Box::new(add_e)),
    );
    instruction_set.insert(
        0x84,
        Instruction::new("add a, h", 0x84, 0, 4, Box::new(add_h)),
    );
    instruction_set.insert(
        0x85,
        Instruction::new("add a, l", 0x85, 0, 4, Box::new(add_l)),
    );
    instruction_set.insert(
        0x86,
        Instruction::new("add a, (hl)", 0x86, 0, 8, Box::new(add_mem_hl)),
    );
    instruction_set.insert(
        0x87,
        Instruction::new("add a, a", 0x87, 0, 4, Box::new(add_a)),
    );
    instruction_set.insert(
        0x88,
        Instruction::new("adc a, b", 0x88, 0, 4, Box::new(adc_b)),
    );
    instruction_set.insert(
        0x89,
        Instruction::new("adc a, c", 0x89, 0, 4, Box::new(adc_c)),
    );
    instruction_set.insert(
        0x8a,
        Instruction::new("adc a, d", 0x8a, 0, 4, Box::new(adc_d)),
    );
    instruction_set.insert(
        0x8b,
        Instruction::new("adc a, e", 0x8b, 0, 4, Box::new(adc_e)),
    );
    instruction_set.insert(
        0x8c,
        Instruction::new("adc a, h", 0x8c, 0, 4, Box::new(adc_h)),
    );
    instruction_set.insert(
        0x8d,
        Instruction::new("adc a, l", 0x8d, 0, 4, Box::new(adc_l)),
    );
    instruction_set.insert(
        0x8e,
        Instruction::new("adc a, (hl)", 0x8e, 0, 8, Box::new(adc_mem_hl)),
    );
    instruction_set.insert(
        0x8f,
        Instruction::new("adc a, a", 0x8f, 0, 4, Box::new(adc_a)),
    );
    instruction_set.insert(
        0x90,
        Instruction::new("sub a, b", 0x90, 0, 4, Box::new(subtract_b)),
    );
    instruction_set.insert(
        0x91,
        Instruction::new("sub a, c", 0x91, 0, 4, Box::new(subtract_c)),
    );
    instruction_set.insert(
        0x92,
        Instruction::new("sub a, d", 0x92, 0, 4, Box::new(subtract_d)),
    );
    instruction_set.insert(
        0x93,
        Instruction::new("sub a, e", 0x93, 0, 4, Box::new(subtract_e)),
    );
    instruction_set.insert(
        0x94,
        Instruction::new("sub a, h", 0x94, 0, 4, Box::new(subtract_h)),
    );
    instruction_set.insert(
        0x95,
        Instruction::new("sub a, l", 0x95, 0, 4, Box::new(subtract_l)),
    );
    instruction_set.insert(
        0x96,
        Instruction::new("sub a, (hl)", 0x96, 0, 8, Box::new(subtract_mem_hl)),
    );
    instruction_set.insert(
        0x97,
        Instruction::new("sub a, a", 0x97, 0, 4, Box::new(subtract_a)),
    );
    instruction_set.insert(
        0x98,
        Instruction::new("sbc a, b", 0x98, 0, 4, Box::new(subtract_b_with_carry)),
    );
    instruction_set.insert(
        0x99,
        Instruction::new("sbc a, c", 0x99, 0, 4, Box::new(subtract_c_with_carry)),
    );
    instruction_set.insert(
        0x9a,
        Instruction::new("sbc a, d", 0x9a, 0, 4, Box::new(subtract_d_with_carry)),
    );
    instruction_set.insert(
        0x9b,
        Instruction::new("sbc a, e", 0x9b, 0, 4, Box::new(subtract_e_with_carry)),
    );
    instruction_set.insert(
        0x9c,
        Instruction::new("sbc a, h", 0x9c, 0, 4, Box::new(subtract_h_with_carry)),
    );
    instruction_set.insert(
        0x9d,
        Instruction::new("sbc a, l", 0x9d, 0, 4, Box::new(subtract_l_with_carry)),
    );
    instruction_set.insert(
        0x9e,
        Instruction::new(
            "sbc a, (hl)",
            0x9e,
            0,
            8,
            Box::new(subtract_mem_hl_with_carry),
        ),
    );
    instruction_set.insert(
        0x9f,
        Instruction::new("sbc a, a", 0x9f, 0, 4, Box::new(subtract_a_with_carry)),
    );
    instruction_set.insert(
        0xa0,
        Instruction::new("and a, b", 0xa0, 0, 4, Box::new(and_b)),
    );
    instruction_set.insert(
        0xa1,
        Instruction::new("and a, c", 0xa1, 0, 4, Box::new(and_c)),
    );
    instruction_set.insert(
        0xa2,
        Instruction::new("and a, d", 0xa2, 0, 4, Box::new(and_d)),
    );
    instruction_set.insert(
        0xa3,
        Instruction::new("and a, e", 0xa3, 0, 4, Box::new(and_e)),
    );
    instruction_set.insert(
        0xa4,
        Instruction::new("and a, h", 0xa4, 0, 4, Box::new(and_h)),
    );
    instruction_set.insert(
        0xa5,
        Instruction::new("and a, l", 0xa5, 0, 4, Box::new(and_l)),
    );
    instruction_set.insert(
        0xa6,
        Instruction::new("and (hl)", 0xa6, 0, 8, Box::new(and_mem_hl)),
    );
    instruction_set.insert(
        0xa7,
        Instruction::new("and a, a", 0xa7, 0, 4, Box::new(and_a)),
    );
    instruction_set.insert(
        0xa8,
        Instruction::new("xor a, b", 0xa8, 0, 4, Box::new(xor_b)),
    );
    instruction_set.insert(
        0xa9,
        Instruction::new("xor a, c", 0xa9, 0, 4, Box::new(xor_c)),
    );
    instruction_set.insert(
        0xaa,
        Instruction::new("xor a, d", 0xaa, 0, 4, Box::new(xor_d)),
    );
    instruction_set.insert(
        0xab,
        Instruction::new("xor a, e", 0xab, 0, 4, Box::new(xor_e)),
    );
    instruction_set.insert(
        0xac,
        Instruction::new("xor a, h", 0xac, 0, 4, Box::new(xor_h)),
    );
    instruction_set.insert(
        0xad,
        Instruction::new("xor a, l", 0xad, 0, 4, Box::new(xor_l)),
    );
    instruction_set.insert(
        0xae,
        Instruction::new("xor a, (hl)", 0xae, 0, 8, Box::new(xor_mem_hl)),
    );
    instruction_set.insert(
        0xaf,
        Instruction::new("xor a, a", 0xaf, 0, 4, Box::new(xor_a)),
    );
    instruction_set.insert(
        0xb0,
        Instruction::new("or a, b", 0xb0, 0, 4, Box::new(or_b)),
    );
    instruction_set.insert(
        0xb1,
        Instruction::new("or a, c", 0xb1, 0, 4, Box::new(or_c)),
    );
    instruction_set.insert(
        0xb2,
        Instruction::new("or a, d", 0xb2, 0, 4, Box::new(or_d)),
    );
    instruction_set.insert(
        0xb3,
        Instruction::new("or a, e", 0xb3, 0, 4, Box::new(or_e)),
    );
    instruction_set.insert(
        0xb4,
        Instruction::new("or a, h", 0xb4, 0, 4, Box::new(or_h)),
    );
    instruction_set.insert(
        0xb5,
        Instruction::new("or a, l", 0xb5, 0, 4, Box::new(or_l)),
    );
    instruction_set.insert(
        0xb6,
        Instruction::new("or a, (hl)", 0xb6, 0, 8, Box::new(or_mem_hl)),
    );
    instruction_set.insert(
        0xb7,
        Instruction::new("or a, a", 0xb7, 0, 4, Box::new(or_a)),
    );
    instruction_set.insert(
        0xb8,
        Instruction::new("cp a, b", 0xb8, 0, 4, Box::new(compare_b)),
    );
    instruction_set.insert(
        0xb9,
        Instruction::new("cp a, c", 0xb9, 0, 4, Box::new(compare_c)),
    );
    instruction_set.insert(
        0xba,
        Instruction::new("cp a, d", 0xba, 0, 4, Box::new(compare_d)),
    );
    instruction_set.insert(
        0xbb,
        Instruction::new("cp a, e", 0xbb, 0, 4, Box::new(compare_e)),
    );
    instruction_set.insert(
        0xbc,
        Instruction::new("cp a, h", 0xbc, 0, 4, Box::new(compare_h)),
    );
    instruction_set.insert(
        0xbd,
        Instruction::new("cp a, l", 0xbd, 0, 4, Box::new(compare_l)),
    );
    instruction_set.insert(
        0xbe,
        Instruction::new("cp a, (hl)", 0xb6, 0, 8, Box::new(compare_mem_hl)),
    );
    instruction_set.insert(
        0xbf,
        Instruction::new("cp a, a", 0xbf, 0, 4, Box::new(compare_a)),
    );
    instruction_set.insert(
        0xc0,
        Instruction::new("ret nz", 0xc0, 0, 8, Box::new(ret_nz)),
    );
    instruction_set.insert(
        0xc1,
        Instruction::new("pop bc", 0xc1, 0, 12, Box::new(pop_bc)),
    );
    instruction_set.insert(
        0xc2,
        Instruction::new("jp nz, a16", 0xc2, 2, 12, Box::new(jp_nz_a16)),
    );
    instruction_set.insert(
        0xc3,
        Instruction::new("JP a16", 0xc3, 2, 12, Box::new(jp_a16)),
    );
    instruction_set.insert(
        0xc4,
        Instruction::new("call nz, a16", 0xc4, 2, 12, Box::new(call_nz_a16)),
    );
    instruction_set.insert(
        0xc5,
        Instruction::new("push bc", 0xc5, 0, 16, Box::new(push_bc)),
    );
    instruction_set.insert(
        0xc6,
        Instruction::new("add d8", 0xc6, 1, 8, Box::new(add_d8)),
    );
    instruction_set.insert(
        0xc7,
        Instruction::new("rst 00h", 0xc7, 0, 32, Box::new(rst_00h)),
    );
    instruction_set.insert(0xc8, Instruction::new("RET Z", 0xc8, 0, 8, Box::new(ret_z)));
    instruction_set.insert(0xc9, Instruction::new("ret", 0xc9, 0, 8, Box::new(ret)));
    instruction_set.insert(
        0xca,
        Instruction::new("jp z, a16", 0xca, 2, 12, Box::new(jp_z_a16)),
    );
    instruction_set.insert(
        0xcc,
        Instruction::new("call z, a16", 0xcc, 2, 12, Box::new(call_z_a16)),
    );
    instruction_set.insert(
        0xcd,
        Instruction::new("call a16", 0xcd, 2, 12, Box::new(call_a16)),
    );
    instruction_set.insert(
        0xce,
        Instruction::new("adc a, d8", 0xce, 1, 8, Box::new(adc_d8)),
    );
    instruction_set.insert(
        0xcf,
        Instruction::new("rst 08h", 0xcf, 0, 32, Box::new(rst_08h)),
    );
    instruction_set.insert(
        0xd0,
        Instruction::new("ret nc", 0xd0, 0, 8, Box::new(ret_nc)),
    );
    instruction_set.insert(
        0xd1,
        Instruction::new("pop de", 0xd1, 0, 12, Box::new(pop_de)),
    );
    instruction_set.insert(
        0xd2,
        Instruction::new("jp nc, a16", 0xd2, 2, 12, Box::new(jp_nc_a16)),
    );
    instruction_set.insert(
        0xd4,
        Instruction::new("call nc, a16", 0xd4, 2, 12, Box::new(call_nc_a16)),
    );
    instruction_set.insert(
        0xd5,
        Instruction::new("push de", 0xd5, 0, 16, Box::new(push_de)),
    );
    instruction_set.insert(
        0xd6,
        Instruction::new("sub d8", 0xd6, 1, 8, Box::new(subtract_d8)),
    );
    instruction_set.insert(
        0xd7,
        Instruction::new("rst 10h", 0xd7, 0, 32, Box::new(rst_10h)),
    );
    instruction_set.insert(0xd8, Instruction::new("ret c", 0xd8, 0, 8, Box::new(ret_c)));
    instruction_set.insert(0xd9, Instruction::new("reti", 0xd9, 0, 8, Box::new(ret_i)));
    instruction_set.insert(
        0xda,
        Instruction::new("jp c, a16", 0xda, 2, 12, Box::new(jp_c_a16)),
    );
    instruction_set.insert(
        0xdc,
        Instruction::new("call c, a16", 0xdc, 2, 12, Box::new(call_c_a16)),
    );
    instruction_set.insert(
        0xde,
        Instruction::new("sbc d8", 0xde, 1, 8, Box::new(subtract_d8_with_carry)),
    );
    instruction_set.insert(
        0xdf,
        Instruction::new("rst 18h", 0xdf, 0, 32, Box::new(rst_18h)),
    );
    instruction_set.insert(
        0xe0,
        Instruction::new("ldh (a8), a", 0xE0, 1, 12, Box::new(ldh_a)),
    );
    instruction_set.insert(
        0xe1,
        Instruction::new("pop hl", 0xe1, 0, 12, Box::new(pop_hl)),
    );
    instruction_set.insert(
        0xe2,
        Instruction::new("ld (c), a", 0xe2, 0, 8, Box::new(ld_mem_c_a)),
    );
    instruction_set.insert(
        0xe5,
        Instruction::new("push hl", 0xe5, 0, 16, Box::new(push_hl)),
    );
    instruction_set.insert(
        0xe6,
        Instruction::new("and d8", 0xe6, 1, 8, Box::new(and_d8)),
    );
    instruction_set.insert(
        0xe7,
        Instruction::new("rst 20h", 0xe7, 0, 32, Box::new(rst_20h)),
    );
    instruction_set.insert(
        0xe8,
        Instruction::new("add sp, r8", 0xe8, 1, 16, Box::new(add_sp_r8)),
    );
    instruction_set.insert(0xe9, Instruction::new("jp hl", 0xe9, 0, 4, Box::new(jp_hl)));
    instruction_set.insert(
        0xea,
        Instruction::new("ld (a16), a", 0xea, 2, 16, Box::new(load_mem_a16_a)),
    );
    instruction_set.insert(
        0xee,
        Instruction::new("xor d8", 0xee, 1, 8, Box::new(xor_d8)),
    );
    instruction_set.insert(
        0xef,
        Instruction::new("rst 28h", 0xef, 0, 32, Box::new(rst_28h)),
    );
    instruction_set.insert(
        0xf0,
        Instruction::new("ldh a, (a8)", 0xf0, 1, 12, Box::new(ldh_a_a8)),
    );
    instruction_set.insert(
        0xf1,
        Instruction::new("pop af", 0xf1, 0, 12, Box::new(pop_af)),
    );
    instruction_set.insert(
        0xf2,
        Instruction::new("ld a, (c)", 0xf2, 0, 8, Box::new(load_a_mem_c)),
    );
    instruction_set.insert(0xf3, Instruction::new("di", 0xf3, 0, 4, Box::new(di)));
    instruction_set.insert(
        0xf5,
        Instruction::new("push af", 0xf5, 0, 16, Box::new(push_af)),
    );
    instruction_set.insert(0xf6, Instruction::new("or d8", 0xf6, 1, 8, Box::new(or_d8)));
    instruction_set.insert(
        0xf7,
        Instruction::new("rst 30h", 0xf7, 0, 32, Box::new(rst_30h)),
    );
    instruction_set.insert(
        0xf8,
        Instruction::new("ld hl, sp + r8", 0xf8, 1, 12, Box::new(load_hl_sp_plus_r8)),
    );
    instruction_set.insert(
        0xf9,
        Instruction::new("ld sp, hl", 0xf9, 0, 8, Box::new(load_sp_hl)),
    );
    instruction_set.insert(
        0xfa,
        Instruction::new("ld a, (a16)", 0xfa, 2, 16, Box::new(load_a_mem_a16)),
    );
    instruction_set.insert(0xfb, Instruction::new("ei", 0xfb, 0, 4, Box::new(ei)));
    instruction_set.insert(
        0xfe,
        Instruction::new("cp d8", 0xfe, 1, 8, Box::new(compare_d8)),
    );
    instruction_set.insert(
        0xff,
        Instruction::new("rst 38h", 0xff, 0, 32, Box::new(rst_38h)),
    );

    cb_instruction_set.insert(0x00, Instruction::new("rlc b", 0x00, 0, 8, Box::new(rlc_b)));
    cb_instruction_set.insert(0x01, Instruction::new("rlc c", 0x01, 0, 8, Box::new(rlc_c)));
    cb_instruction_set.insert(0x02, Instruction::new("rlc d", 0x02, 0, 8, Box::new(rlc_d)));
    cb_instruction_set.insert(0x03, Instruction::new("rlc e", 0x03, 0, 8, Box::new(rlc_e)));
    cb_instruction_set.insert(0x04, Instruction::new("rlc h", 0x04, 0, 8, Box::new(rlc_h)));
    cb_instruction_set.insert(0x05, Instruction::new("rlc l", 0x05, 0, 8, Box::new(rlc_l)));
    cb_instruction_set.insert(
        0x06,
        Instruction::new("rlc (hl)", 0x06, 0, 16, Box::new(rlc_mem_hl)),
    );
    cb_instruction_set.insert(0x07, Instruction::new("rlc a", 0x07, 0, 8, Box::new(rlc_a)));
    cb_instruction_set.insert(0x08, Instruction::new("rrc b", 0x08, 0, 8, Box::new(rrc_b)));
    cb_instruction_set.insert(0x09, Instruction::new("rrc c", 0x09, 0, 8, Box::new(rrc_c)));
    cb_instruction_set.insert(0x0a, Instruction::new("rrc d", 0x0a, 0, 8, Box::new(rrc_d)));
    cb_instruction_set.insert(0x0b, Instruction::new("rrc e", 0x0b, 0, 8, Box::new(rrc_e)));
    cb_instruction_set.insert(0x0c, Instruction::new("rrc h", 0x0c, 0, 8, Box::new(rrc_h)));
    cb_instruction_set.insert(0x0d, Instruction::new("rrc l", 0x0d, 0, 8, Box::new(rrc_l)));
    cb_instruction_set.insert(
        0x0e,
        Instruction::new("rrc (hl)", 0x0e, 0, 16, Box::new(rrc_mem_hl)),
    );
    cb_instruction_set.insert(0x0f, Instruction::new("rrc a", 0x0f, 0, 8, Box::new(rrc_a)));
    cb_instruction_set.insert(0x10, Instruction::new("rl b", 0x10, 0, 8, Box::new(rl_b)));
    cb_instruction_set.insert(0x11, Instruction::new("rl c", 0x11, 0, 8, Box::new(rl_c)));
    cb_instruction_set.insert(0x12, Instruction::new("rl d", 0x12, 0, 8, Box::new(rl_d)));
    cb_instruction_set.insert(0x13, Instruction::new("rl e", 0x13, 0, 8, Box::new(rl_e)));
    cb_instruction_set.insert(0x14, Instruction::new("rl h", 0x14, 0, 8, Box::new(rl_h)));
    cb_instruction_set.insert(0x15, Instruction::new("rl l", 0x15, 0, 8, Box::new(rl_l)));
    cb_instruction_set.insert(
        0x16,
        Instruction::new("rl (hl)", 0x16, 0, 16, Box::new(rl_mem_hl)),
    );
    cb_instruction_set.insert(0x17, Instruction::new("rl a", 0x17, 0, 8, Box::new(rl_a)));
    cb_instruction_set.insert(0x18, Instruction::new("rr b", 0x18, 0, 8, Box::new(rr_b)));
    cb_instruction_set.insert(0x19, Instruction::new("rr c", 0x19, 0, 8, Box::new(rr_c)));
    cb_instruction_set.insert(0x1a, Instruction::new("rr d", 0x1a, 0, 8, Box::new(rr_d)));
    cb_instruction_set.insert(0x1b, Instruction::new("rr e", 0x1b, 0, 8, Box::new(rr_e)));
    cb_instruction_set.insert(0x1c, Instruction::new("rr h", 0x1c, 0, 8, Box::new(rr_h)));
    cb_instruction_set.insert(0x1d, Instruction::new("rr l", 0x1c, 0, 8, Box::new(rr_l)));
    cb_instruction_set.insert(
        0x1e,
        Instruction::new("rr (hl)", 0x1c, 0, 16, Box::new(rr_mem_hl)),
    );
    cb_instruction_set.insert(0x1f, Instruction::new("rr a", 0x1f, 0, 8, Box::new(rr_a)));
    cb_instruction_set.insert(0x20, Instruction::new("sla b", 0x20, 0, 8, Box::new(sla_b)));
    cb_instruction_set.insert(0x21, Instruction::new("sla c", 0x21, 0, 8, Box::new(sla_c)));
    cb_instruction_set.insert(0x22, Instruction::new("sla d", 0x22, 0, 8, Box::new(sla_d)));
    cb_instruction_set.insert(0x23, Instruction::new("sla e", 0x23, 0, 8, Box::new(sla_e)));
    cb_instruction_set.insert(0x24, Instruction::new("sla h", 0x24, 0, 8, Box::new(sla_h)));
    cb_instruction_set.insert(0x25, Instruction::new("sla l", 0x25, 0, 8, Box::new(sla_l)));
    cb_instruction_set.insert(
        0x26,
        Instruction::new("sla (hl)", 0x26, 0, 16, Box::new(sla_mem_hl)),
    );
    cb_instruction_set.insert(0x27, Instruction::new("sla a", 0x27, 0, 8, Box::new(sla_a)));
    cb_instruction_set.insert(0x28, Instruction::new("sra b", 0x28, 0, 8, Box::new(sra_b)));
    cb_instruction_set.insert(0x29, Instruction::new("sra c", 0x29, 0, 8, Box::new(sra_c)));
    cb_instruction_set.insert(0x2a, Instruction::new("sra d", 0x2a, 0, 8, Box::new(sra_d)));
    cb_instruction_set.insert(0x2b, Instruction::new("sra e", 0x2b, 0, 8, Box::new(sra_e)));
    cb_instruction_set.insert(0x2c, Instruction::new("sra h", 0x2c, 0, 8, Box::new(sra_h)));
    cb_instruction_set.insert(0x2d, Instruction::new("sra l", 0x2d, 0, 8, Box::new(sra_l)));
    cb_instruction_set.insert(
        0x2e,
        Instruction::new("sra (hl)", 0x2e, 0, 16, Box::new(sra_mem_hl)),
    );
    cb_instruction_set.insert(0x2f, Instruction::new("sra a", 0x2f, 0, 8, Box::new(sra_a)));
    cb_instruction_set.insert(
        0x30,
        Instruction::new("swap b", 0x30, 0, 8, Box::new(swap_b)),
    );
    cb_instruction_set.insert(
        0x31,
        Instruction::new("swap c", 0x31, 0, 8, Box::new(swap_c)),
    );
    cb_instruction_set.insert(
        0x32,
        Instruction::new("swap d", 0x32, 0, 8, Box::new(swap_d)),
    );
    cb_instruction_set.insert(
        0x33,
        Instruction::new("swap e", 0x33, 0, 8, Box::new(swap_e)),
    );
    cb_instruction_set.insert(
        0x34,
        Instruction::new("swap h", 0x34, 0, 8, Box::new(swap_h)),
    );
    cb_instruction_set.insert(
        0x35,
        Instruction::new("swap l", 0x35, 0, 8, Box::new(swap_l)),
    );
    cb_instruction_set.insert(
        0x36,
        Instruction::new("swap (hl)", 0x36, 0, 16, Box::new(swap_mem_hl)),
    );
    cb_instruction_set.insert(
        0x37,
        Instruction::new("swap a", 0x37, 0, 8, Box::new(swap_a)),
    );
    cb_instruction_set.insert(0x38, Instruction::new("srl b", 0x38, 0, 8, Box::new(srl_b)));
    cb_instruction_set.insert(0x39, Instruction::new("srl c", 0x39, 0, 8, Box::new(srl_c)));
    cb_instruction_set.insert(0x3a, Instruction::new("srl d", 0x3a, 0, 8, Box::new(srl_d)));
    cb_instruction_set.insert(0x3b, Instruction::new("srl e", 0x3b, 0, 8, Box::new(srl_e)));
    cb_instruction_set.insert(0x3c, Instruction::new("srl h", 0x3c, 0, 8, Box::new(srl_h)));
    cb_instruction_set.insert(0x3d, Instruction::new("srl l", 0x3d, 0, 8, Box::new(srl_l)));
    cb_instruction_set.insert(
        0x3e,
        Instruction::new("srl (hl)", 0x3e, 0, 16, Box::new(srl_mem_hl)),
    );
    cb_instruction_set.insert(0x3f, Instruction::new("srl a", 0x3f, 0, 8, Box::new(srl_a)));
    cb_instruction_set.insert(
        0x40,
        Instruction::new("bit 0, b", 0x40, 0, 8, Box::new(bit_0_b)),
    );
    cb_instruction_set.insert(
        0x41,
        Instruction::new("bit 0, c", 0x41, 0, 8, Box::new(bit_0_c)),
    );
    cb_instruction_set.insert(
        0x42,
        Instruction::new("bit 0, d", 0x42, 0, 8, Box::new(bit_0_d)),
    );
    cb_instruction_set.insert(
        0x43,
        Instruction::new("bit 0, e", 0x43, 0, 8, Box::new(bit_0_e)),
    );
    cb_instruction_set.insert(
        0x44,
        Instruction::new("bit 0, h", 0x44, 0, 8, Box::new(bit_0_h)),
    );
    cb_instruction_set.insert(
        0x45,
        Instruction::new("bit 0, l", 0x45, 0, 8, Box::new(bit_0_l)),
    );
    cb_instruction_set.insert(
        0x46,
        Instruction::new("bit 0, (hl)", 0x46, 0, 16, Box::new(bit_0_mem_hl)),
    );
    cb_instruction_set.insert(
        0x47,
        Instruction::new("bit 0, a", 0x47, 0, 8, Box::new(bit_0_a)),
    );
    cb_instruction_set.insert(
        0x48,
        Instruction::new("bit 1, b", 0x48, 0, 8, Box::new(bit_1_b)),
    );
    cb_instruction_set.insert(
        0x49,
        Instruction::new("bit 1, c", 0x49, 0, 8, Box::new(bit_1_c)),
    );
    cb_instruction_set.insert(
        0x4a,
        Instruction::new("bit 1, d", 0x4a, 0, 8, Box::new(bit_1_d)),
    );
    cb_instruction_set.insert(
        0x4b,
        Instruction::new("bit 1, e", 0x4b, 0, 8, Box::new(bit_1_e)),
    );
    cb_instruction_set.insert(
        0x4c,
        Instruction::new("bit 1, h", 0x4c, 0, 8, Box::new(bit_1_h)),
    );
    cb_instruction_set.insert(
        0x4d,
        Instruction::new("bit 1, l", 0x4d, 0, 8, Box::new(bit_1_l)),
    );
    cb_instruction_set.insert(
        0x4e,
        Instruction::new("bit 1, (hl)", 0x4e, 0, 16, Box::new(bit_1_mem_hl)),
    );
    cb_instruction_set.insert(
        0x4f,
        Instruction::new("bit 1, a", 0x4f, 0, 8, Box::new(bit_1_a)),
    );
    cb_instruction_set.insert(
        0x50,
        Instruction::new("bit 2, b", 0x50, 0, 8, Box::new(bit_2_b)),
    );
    cb_instruction_set.insert(
        0x51,
        Instruction::new("bit 2, c", 0x51, 0, 8, Box::new(bit_2_c)),
    );
    cb_instruction_set.insert(
        0x52,
        Instruction::new("bit 2, d", 0x52, 0, 8, Box::new(bit_2_d)),
    );
    cb_instruction_set.insert(
        0x53,
        Instruction::new("bit 2, e", 0x53, 0, 8, Box::new(bit_2_e)),
    );
    cb_instruction_set.insert(
        0x54,
        Instruction::new("bit 2, h", 0x54, 0, 8, Box::new(bit_2_h)),
    );
    cb_instruction_set.insert(
        0x55,
        Instruction::new("bit 2, l", 0x55, 0, 8, Box::new(bit_2_l)),
    );
    cb_instruction_set.insert(
        0x56,
        Instruction::new("bit 2, (hl)", 0x56, 0, 16, Box::new(bit_2_mem_hl)),
    );
    cb_instruction_set.insert(
        0x57,
        Instruction::new("bit 2, a", 0x57, 0, 8, Box::new(bit_2_a)),
    );
    cb_instruction_set.insert(
        0x58,
        Instruction::new("bit 3, b", 0x58, 0, 8, Box::new(bit_3_b)),
    );
    cb_instruction_set.insert(
        0x59,
        Instruction::new("bit 3, c", 0x59, 0, 8, Box::new(bit_3_c)),
    );
    cb_instruction_set.insert(
        0x5a,
        Instruction::new("bit 3, d", 0x5a, 0, 8, Box::new(bit_3_d)),
    );
    cb_instruction_set.insert(
        0x5b,
        Instruction::new("bit 3, e", 0x5b, 0, 8, Box::new(bit_3_e)),
    );
    cb_instruction_set.insert(
        0x5c,
        Instruction::new("bit 3, h", 0x5c, 0, 8, Box::new(bit_3_h)),
    );
    cb_instruction_set.insert(
        0x5d,
        Instruction::new("bit 3, l", 0x5d, 0, 8, Box::new(bit_3_l)),
    );
    cb_instruction_set.insert(
        0x5e,
        Instruction::new("bit 3, (hl)", 0x5e, 0, 16, Box::new(bit_3_mem_hl)),
    );
    cb_instruction_set.insert(
        0x5f,
        Instruction::new("bit 3, a", 0x5f, 0, 8, Box::new(bit_3_a)),
    );
    cb_instruction_set.insert(
        0x60,
        Instruction::new("bit 4, b", 0x60, 0, 8, Box::new(bit_4_b)),
    );
    cb_instruction_set.insert(
        0x61,
        Instruction::new("bit 4, c", 0x61, 0, 8, Box::new(bit_4_c)),
    );
    cb_instruction_set.insert(
        0x62,
        Instruction::new("bit 4, d", 0x62, 0, 8, Box::new(bit_4_d)),
    );
    cb_instruction_set.insert(
        0x63,
        Instruction::new("bit 4, e", 0x63, 0, 8, Box::new(bit_4_e)),
    );
    cb_instruction_set.insert(
        0x64,
        Instruction::new("bit 4, h", 0x64, 0, 8, Box::new(bit_4_h)),
    );
    cb_instruction_set.insert(
        0x65,
        Instruction::new("bit 4, l", 0x65, 0, 8, Box::new(bit_4_l)),
    );
    cb_instruction_set.insert(
        0x66,
        Instruction::new("bit 4, (hl)", 0x66, 0, 16, Box::new(bit_4_mem_hl)),
    );
    cb_instruction_set.insert(
        0x67,
        Instruction::new("bit 4, a", 0x67, 0, 8, Box::new(bit_4_a)),
    );
    cb_instruction_set.insert(
        0x68,
        Instruction::new("bit 5, b", 0x68, 0, 8, Box::new(bit_5_b)),
    );
    cb_instruction_set.insert(
        0x69,
        Instruction::new("bit 5, c", 0x69, 0, 8, Box::new(bit_5_c)),
    );
    cb_instruction_set.insert(
        0x6a,
        Instruction::new("bit 5, d", 0x6a, 0, 8, Box::new(bit_5_d)),
    );
    cb_instruction_set.insert(
        0x6b,
        Instruction::new("bit 5, e", 0x6b, 0, 8, Box::new(bit_5_e)),
    );
    cb_instruction_set.insert(
        0x6c,
        Instruction::new("bit 5, h", 0x6c, 0, 8, Box::new(bit_5_h)),
    );
    cb_instruction_set.insert(
        0x6d,
        Instruction::new("bit 5, l", 0x6d, 0, 8, Box::new(bit_5_l)),
    );
    cb_instruction_set.insert(
        0x6e,
        Instruction::new("bit 5, (hl)", 0x6e, 0, 16, Box::new(bit_5_mem_hl)),
    );
    cb_instruction_set.insert(
        0x6f,
        Instruction::new("bit 5, a", 0x6f, 0, 8, Box::new(bit_5_a)),
    );

    (instruction_set, cb_instruction_set)
}

fn bit_5_a(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.a, 5);
}

fn bit_5_b(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.b, 5);
}

fn bit_5_c(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.c, 5);
}

fn bit_5_d(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.d, 5);
}

fn bit_5_e(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.e, 5);
}

fn bit_5_h(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.h, 5);
}

fn bit_5_l(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.l, 5);
}

fn bit_5_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_bit(value, 5);
}

fn bit_4_a(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.a, 4);
}

fn bit_4_b(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.b, 4);
}

fn bit_4_c(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.c, 4);
}

fn bit_4_d(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.d, 4);
}

fn bit_4_e(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.e, 4);
}

fn bit_4_h(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.h, 4);
}

fn bit_4_l(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.l, 4);
}

fn bit_4_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_bit(value, 4);
}

fn bit_3_a(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.a, 3);
}

fn bit_3_b(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.b, 3);
}

fn bit_3_c(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.c, 3);
}

fn bit_3_d(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.d, 3);
}

fn bit_3_e(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.e, 3);
}

fn bit_3_h(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.h, 3);
}

fn bit_3_l(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.l, 3);
}

fn bit_3_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_bit(value, 3);
}

fn bit_2_a(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.a, 2);
}

fn bit_2_b(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.b, 2);
}

fn bit_2_c(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.c, 2);
}

fn bit_2_d(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.d, 2);
}

fn bit_2_e(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.e, 2);
}

fn bit_2_h(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.h, 2);
}

fn bit_2_l(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.l, 2);
}

fn bit_2_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_bit(value, 2);
}

fn bit_1_a(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.a, 1);
}

fn bit_1_b(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.b, 1);
}

fn bit_1_c(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.c, 1);
}

fn bit_1_d(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.d, 1);
}

fn bit_1_e(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.e, 1);
}

fn bit_1_h(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.h, 1);
}

fn bit_1_l(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.l, 1);
}

fn bit_1_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_bit(value, 1);
}

fn bit_0_a(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.a, 0);
}

fn bit_0_b(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.b, 0);
}

fn bit_0_c(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.c, 0);
}

fn bit_0_d(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.d, 0);
}

fn bit_0_e(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.e, 0);
}

fn bit_0_h(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.h, 0);
}

fn bit_0_l(core: &mut Core, _: Option<Operand>) {
    core.alu_bit(core.registers.l, 0);
}

fn bit_0_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_bit(value, 0);
}

fn srl_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_srl(core.registers.a);
}

fn srl_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_srl(core.registers.b);
}

fn srl_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_srl(core.registers.c);
}

fn srl_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_srl(core.registers.d);
}

fn srl_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_srl(core.registers.e);
}

fn srl_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_srl(core.registers.h);
}

fn srl_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_srl(core.registers.l);
}

fn srl_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let value = core.memory.borrow().get(address);
    let result = core.alu_srl(value);
    core.memory.borrow_mut().set(address, result);
}

fn swap_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_swap(core.registers.a);
}

fn swap_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_swap(core.registers.b);
}

fn swap_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_swap(core.registers.c);
}

fn swap_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_swap(core.registers.d);
}

fn swap_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_swap(core.registers.e);
}

fn swap_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_swap(core.registers.h);
}

fn swap_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_swap(core.registers.l);
}

fn swap_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let value = core.memory.borrow().get(address);
    let result = core.alu_swap(value);
    core.memory.borrow_mut().set(address, result);
}

fn sra_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_sra(core.registers.a);
}

fn sra_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_sra(core.registers.b);
}

fn sra_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_sra(core.registers.c);
}

fn sra_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_sra(core.registers.d);
}

fn sra_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_sra(core.registers.e);
}

fn sra_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_sra(core.registers.h);
}

fn sra_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_sra(core.registers.l);
}

fn sra_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let value = core.memory.borrow().get(address);
    let result = core.alu_sra(value);
    core.memory.borrow_mut().set(address, result);
}

fn sla_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_sla(core.registers.a);
}

fn sla_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_sla(core.registers.b);
}

fn sla_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_sla(core.registers.c);
}

fn sla_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_sla(core.registers.d);
}

fn sla_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_sla(core.registers.e);
}

fn sla_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_sla(core.registers.h);
}

fn sla_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_sla(core.registers.l);
}

fn sla_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let value = core.memory.borrow().get(address);
    let result = core.alu_sla(value);
    core.memory.borrow_mut().set(address, result);
}

fn rr_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_rr(core.registers.a);
}

fn rr_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_rr(core.registers.b);
}

fn rr_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_rr(core.registers.c);
}

fn rr_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_rr(core.registers.d);
}

fn rr_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_rr(core.registers.e);
}

fn rr_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_rr(core.registers.h);
}

fn rr_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_rr(core.registers.l);
}

fn rr_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let value = core.memory.borrow().get(address);
    let result = core.alu_rr(value);
    core.memory.borrow_mut().set(address, result);
}

fn rl_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_rl(core.registers.a);
}

fn rl_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_rl(core.registers.b);
}

fn rl_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_rl(core.registers.c);
}

fn rl_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_rl(core.registers.d);
}

fn rl_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_rl(core.registers.e);
}

fn rl_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_rl(core.registers.h);
}

fn rl_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_rl(core.registers.l);
}

fn rl_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let value = core.memory.borrow().get(address);
    let result = core.alu_rl(value);
    core.memory.borrow_mut().set(address, result);
}

fn rrc_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_rrc(core.registers.a);
}

fn rrc_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_rrc(core.registers.b);
}

fn rrc_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_rrc(core.registers.c);
}

fn rrc_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_rrc(core.registers.d);
}

fn rrc_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_rrc(core.registers.e);
}

fn rrc_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_rrc(core.registers.h);
}

fn rrc_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_rrc(core.registers.l);
}

fn rrc_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let value = core.memory.borrow().get(address);
    let result = core.alu_rrc(value);
    core.memory.borrow_mut().set(address, result);
}

fn rlc_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_rlc(core.registers.a);
}

fn rlc_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_rlc(core.registers.b);
}

fn rlc_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_rlc(core.registers.c);
}

fn rlc_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_rlc(core.registers.d);
}

fn rlc_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_rlc(core.registers.e);
}

fn rlc_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_rlc(core.registers.h);
}

fn rlc_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_rlc(core.registers.l);
}

fn rlc_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let value = core.memory.borrow().get(address);
    let result = core.alu_rlc(value);
    core.memory.borrow_mut().set(address, result);
}

fn load_hl_sp_plus_r8(core: &mut Core, operand: Option<Operand>) {
    let a = core.registers.sp;
    let b = i16::from(operand.unwrap().byte as i8) as u16;
    core.registers
        .set_flag(Flag::C, (a & 0x00ff) + (b & 0x00ff) > 0x00ff);
    core.registers
        .set_flag(Flag::H, (a & 0x000f) + (b & 0x000f) > 0x000f);
    core.registers.set_flag(Flag::N, false);
    core.registers.set_flag(Flag::Z, false);
    core.registers.set_hl(a.wrapping_add(b));
}

fn di(core: &mut Core, _: Option<Operand>) {
    core.ei = false;
}

fn ei(core: &mut Core, _: Option<Operand>) {
    core.ei = true;
}

fn load_mem_a16_a(core: &mut Core, operand: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(operand.unwrap().word, core.registers.a);
}

fn load_a_mem_c(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core
        .memory
        .borrow()
        .get(0xff00 | u16::from(core.registers.c));
}

fn load_a_mem_a16(core: &mut Core, operand: Option<Operand>) {
    core.registers.a = core.memory.borrow().get(operand.unwrap().word);
}

fn ldh_a(core: &mut Core, operand: Option<Operand>) {
    let address = 0xff00 | u16::from(operand.unwrap().byte);
    core.memory.borrow_mut().set(address, core.registers.a);
}

fn ldh_a_a8(core: &mut Core, operand: Option<Operand>) {
    let address = 0xff00 | u16::from(operand.unwrap().byte);
    core.registers.a = core.memory.borrow().get(address);
}

fn ld_mem_c_a(core: &mut Core, _: Option<Operand>) {
    let address = 0xff00 | u16::from(core.registers.c);
    core.memory.borrow_mut().set(address, core.registers.a);
}

fn rst_00h(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = 0x00;
}

fn rst_08h(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = 0x08;
}

fn rst_10h(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = 0x10;
}

fn rst_18h(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = 0x18;
}

fn rst_20h(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = 0x20;
}

fn rst_28h(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = 0x28;
}

fn rst_30h(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = 0x30;
}

fn rst_38h(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = 0x38;
}

fn push_bc(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.get_bc());
}

fn push_hl(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.get_hl());
}

fn push_af(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.get_af());
}

fn push_de(core: &mut Core, _: Option<Operand>) {
    core.stack_push(core.registers.get_de());
}

fn call_nc_a16(core: &mut Core, operand: Option<Operand>) {
    if !core.registers.get_flag(Flag::C) {
        core.stack_push(core.registers.pc);
        core.registers.pc = operand.unwrap().word;
    }
}

fn call_c_a16(core: &mut Core, operand: Option<Operand>) {
    if core.registers.get_flag(Flag::C) {
        core.stack_push(core.registers.pc);
        core.registers.pc = operand.unwrap().word;
    }
}

fn call_nz_a16(core: &mut Core, operand: Option<Operand>) {
    if !core.registers.get_flag(Flag::Z) {
        core.stack_push(core.registers.pc);
        core.registers.pc = operand.unwrap().word;
    }
}

fn call_a16(core: &mut Core, operand: Option<Operand>) {
    core.stack_push(core.registers.pc);
    core.registers.pc = operand.unwrap().word;
}

fn call_z_a16(core: &mut Core, operand: Option<Operand>) {
    if core.registers.get_flag(Flag::Z) {
        core.stack_push(core.registers.pc);
        core.registers.pc = operand.unwrap().word;
    }
}

fn jp_c_a16(core: &mut Core, operand: Option<Operand>) {
    if core.registers.get_flag(Flag::C) {
        core.registers.pc = operand.unwrap().word;
    }
}

fn jp_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.pc = core.registers.get_hl();
}

fn jp_nz_a16(core: &mut Core, operand: Option<Operand>) {
    if !core.registers.get_flag(Flag::Z) {
        core.registers.pc = operand.unwrap().word;
    }
}

fn jp_nc_a16(core: &mut Core, operand: Option<Operand>) {
    if !core.registers.get_flag(Flag::C) {
        core.registers.pc = operand.unwrap().word;
    }
}

fn jp_z_a16(core: &mut Core, operand: Option<Operand>) {
    if core.registers.get_flag(Flag::Z) {
        core.registers.pc = operand.unwrap().word;
    }
}

fn jp_a16(core: &mut Core, operand: Option<Operand>) {
    core.registers.pc = operand.unwrap().word;
}

fn pop_bc(core: &mut Core, _: Option<Operand>) {
    let value = core.stack_pop();
    core.registers.set_bc(value);
}

fn pop_af(core: &mut Core, _: Option<Operand>) {
    let value = core.stack_pop();
    core.registers.set_af(value);
}

fn pop_de(core: &mut Core, _: Option<Operand>) {
    let value = core.stack_pop();
    core.registers.set_de(value);
}

fn pop_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.stack_pop();
    core.registers.set_hl(value);
}

fn ret(core: &mut Core, _: Option<Operand>) {
    core.registers.pc = core.stack_pop();
}

fn ret_i(core: &mut Core, _: Option<Operand>) {
    core.registers.pc = core.stack_pop();
    core.ei = true;
}

fn ret_nz(core: &mut Core, _: Option<Operand>) {
    if !core.registers.get_flag(Flag::Z) {
        core.registers.pc = core.stack_pop();
    }
}

fn ret_nc(core: &mut Core, _: Option<Operand>) {
    if !core.registers.get_flag(Flag::C) {
        core.registers.pc = core.stack_pop();
    }
}

fn ret_c(core: &mut Core, _: Option<Operand>) {
    if core.registers.get_flag(Flag::C) {
        core.registers.pc = core.stack_pop();
    }
}

fn ret_z(core: &mut Core, _: Option<Operand>) {
    if core.registers.get_flag(Flag::Z) {
        core.registers.pc = core.stack_pop();
    }
}

fn compare_d8(core: &mut Core, operand: Option<Operand>) {
    core.alu_cp(operand.unwrap().byte);
}

fn compare_a(core: &mut Core, _: Option<Operand>) {
    core.alu_cp(core.registers.a);
}

fn compare_b(core: &mut Core, _: Option<Operand>) {
    core.alu_cp(core.registers.b);
}

fn compare_c(core: &mut Core, _: Option<Operand>) {
    core.alu_cp(core.registers.c);
}

fn compare_d(core: &mut Core, _: Option<Operand>) {
    core.alu_cp(core.registers.d);
}

fn compare_e(core: &mut Core, _: Option<Operand>) {
    core.alu_cp(core.registers.e);
}

fn compare_h(core: &mut Core, _: Option<Operand>) {
    core.alu_cp(core.registers.h);
}

fn compare_l(core: &mut Core, _: Option<Operand>) {
    core.alu_cp(core.registers.l);
}

fn compare_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_cp(value);
}

fn or_d8(core: &mut Core, operand: Option<Operand>) {
    core.alu_or(operand.unwrap().byte);
}

fn or_a(core: &mut Core, _: Option<Operand>) {
    core.alu_or(core.registers.a);
}

fn or_b(core: &mut Core, _: Option<Operand>) {
    core.alu_or(core.registers.b);
}

fn or_c(core: &mut Core, _: Option<Operand>) {
    core.alu_or(core.registers.c);
}

fn or_d(core: &mut Core, _: Option<Operand>) {
    core.alu_or(core.registers.d);
}

fn or_e(core: &mut Core, _: Option<Operand>) {
    core.alu_or(core.registers.e);
}

fn or_h(core: &mut Core, _: Option<Operand>) {
    core.alu_or(core.registers.h);
}

fn or_l(core: &mut Core, _: Option<Operand>) {
    core.alu_or(core.registers.l);
}

fn or_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_or(value);
}

fn xor_d8(core: &mut Core, operand: Option<Operand>) {
    core.alu_xor(operand.unwrap().byte);
}

fn xor_a(core: &mut Core, _: Option<Operand>) {
    core.alu_xor(core.registers.a);
}

fn xor_b(core: &mut Core, _: Option<Operand>) {
    core.alu_xor(core.registers.b);
}

fn xor_c(core: &mut Core, _: Option<Operand>) {
    core.alu_xor(core.registers.c);
}

fn xor_d(core: &mut Core, _: Option<Operand>) {
    core.alu_xor(core.registers.d);
}

fn xor_e(core: &mut Core, _: Option<Operand>) {
    core.alu_xor(core.registers.e);
}

fn xor_h(core: &mut Core, _: Option<Operand>) {
    core.alu_xor(core.registers.h);
}

fn xor_l(core: &mut Core, _: Option<Operand>) {
    core.alu_xor(core.registers.l);
}

fn xor_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_xor(value);
}

fn and_d8(core: &mut Core, operand: Option<Operand>) {
    core.alu_and(operand.unwrap().byte);
}

fn and_a(core: &mut Core, _: Option<Operand>) {
    core.alu_and(core.registers.a);
}

fn and_b(core: &mut Core, _: Option<Operand>) {
    core.alu_and(core.registers.b);
}

fn and_c(core: &mut Core, _: Option<Operand>) {
    core.alu_and(core.registers.c);
}

fn and_d(core: &mut Core, _: Option<Operand>) {
    core.alu_and(core.registers.d);
}

fn and_e(core: &mut Core, _: Option<Operand>) {
    core.alu_and(core.registers.e);
}

fn and_h(core: &mut Core, _: Option<Operand>) {
    core.alu_and(core.registers.h);
}

fn and_l(core: &mut Core, _: Option<Operand>) {
    core.alu_and(core.registers.l);
}

fn and_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_and(value);
}

fn subtract_d8_with_carry(core: &mut Core, operand: Option<Operand>) {
    core.alu_sbc(operand.unwrap().byte);
}

fn subtract_a_with_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_sbc(core.registers.a);
}

fn subtract_b_with_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_sbc(core.registers.b);
}

fn subtract_c_with_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_sbc(core.registers.c);
}

fn subtract_d_with_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_sbc(core.registers.d);
}

fn subtract_e_with_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_sbc(core.registers.e);
}

fn subtract_h_with_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_sbc(core.registers.h);
}

fn subtract_l_with_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_sbc(core.registers.l);
}

fn subtract_mem_hl_with_carry(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_sbc(value);
}

fn subtract_d8(core: &mut Core, operand: Option<Operand>) {
    core.alu_sub(operand.unwrap().byte);
}

fn subtract_a(core: &mut Core, _: Option<Operand>) {
    core.alu_sub(core.registers.a);
}

fn subtract_b(core: &mut Core, _: Option<Operand>) {
    core.alu_sub(core.registers.b);
}

fn subtract_c(core: &mut Core, _: Option<Operand>) {
    core.alu_sub(core.registers.c);
}

fn subtract_d(core: &mut Core, _: Option<Operand>) {
    core.alu_sub(core.registers.d);
}

fn subtract_e(core: &mut Core, _: Option<Operand>) {
    core.alu_sub(core.registers.e);
}

fn subtract_h(core: &mut Core, _: Option<Operand>) {
    core.alu_sub(core.registers.h);
}

fn subtract_l(core: &mut Core, _: Option<Operand>) {
    core.alu_sub(core.registers.l);
}

fn subtract_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_sub(value);
}

fn adc_d8(core: &mut Core, operand: Option<Operand>) {
    core.alu_adc(operand.unwrap().byte);
}

fn adc_a(core: &mut Core, _: Option<Operand>) {
    core.alu_adc(core.registers.a);
}

fn adc_b(core: &mut Core, _: Option<Operand>) {
    core.alu_adc(core.registers.b);
}

fn adc_c(core: &mut Core, _: Option<Operand>) {
    core.alu_adc(core.registers.c);
}

fn adc_d(core: &mut Core, _: Option<Operand>) {
    core.alu_adc(core.registers.d);
}

fn adc_e(core: &mut Core, _: Option<Operand>) {
    core.alu_adc(core.registers.e);
}

fn adc_h(core: &mut Core, _: Option<Operand>) {
    core.alu_adc(core.registers.h);
}

fn adc_l(core: &mut Core, _: Option<Operand>) {
    core.alu_adc(core.registers.l);
}

fn adc_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_adc(value);
}

fn add_d8(core: &mut Core, operand: Option<Operand>) {
    core.alu_add(operand.unwrap().byte);
}

fn add_sp_r8(core: &mut Core, operand: Option<Operand>) {
    core.alu_add_sp(operand.unwrap().byte);
}

fn add_a(core: &mut Core, _: Option<Operand>) {
    core.alu_add(core.registers.a);
}

fn add_b(core: &mut Core, _: Option<Operand>) {
    core.alu_add(core.registers.b);
}

fn add_c(core: &mut Core, _: Option<Operand>) {
    core.alu_add(core.registers.c);
}

fn add_d(core: &mut Core, _: Option<Operand>) {
    core.alu_add(core.registers.d);
}

fn add_e(core: &mut Core, _: Option<Operand>) {
    core.alu_add(core.registers.e);
}

fn add_h(core: &mut Core, _: Option<Operand>) {
    core.alu_add(core.registers.h);
}

fn add_l(core: &mut Core, _: Option<Operand>) {
    core.alu_add(core.registers.l);
}

fn add_mem_hl(core: &mut Core, _: Option<Operand>) {
    let value = core.memory.borrow().get(core.registers.get_hl());
    core.alu_add(value);
}

fn load_a_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.registers.a;
}

fn load_a_b(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.registers.b;
}

fn load_a_c(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.registers.c;
}

fn load_a_d(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.registers.d;
}

fn load_a_e(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.registers.e;
}

fn load_a_h(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.registers.h;
}

fn load_a_l(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.registers.l;
}

fn load_l_a(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.registers.a;
}

fn load_l_b(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.registers.b;
}

fn load_l_c(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.registers.c;
}

fn load_l_d(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.registers.d;
}

fn load_l_e(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.registers.e;
}

fn load_l_h(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.registers.h;
}

fn load_l_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.registers.l;
}

fn load_h_a(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.registers.a;
}

fn load_h_b(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.registers.b;
}

fn load_h_c(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.registers.c;
}

fn load_h_d(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.registers.d;
}

fn load_h_e(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.registers.e;
}

fn load_h_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.registers.h;
}

fn load_h_l(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.registers.l;
}

fn load_e_a(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.registers.a;
}

fn load_e_b(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.registers.b;
}

fn load_e_c(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.registers.c;
}

fn load_e_d(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.registers.d;
}

fn load_e_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.registers.e;
}

fn load_e_h(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.registers.h;
}

fn load_e_l(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.registers.l;
}

fn load_d_a(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.registers.a;
}

fn load_d_b(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.registers.b;
}

fn load_d_c(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.registers.c;
}

fn load_d_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.registers.d;
}

fn load_d_e(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.registers.e;
}

fn load_d_h(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.registers.h;
}

fn load_d_l(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.registers.l;
}

fn load_c_a(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.registers.a;
}

fn load_c_b(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.registers.b;
}

fn load_c_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.registers.c;
}

fn load_c_d(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.registers.d;
}

fn load_c_e(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.registers.e;
}

fn load_c_h(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.registers.h;
}

fn load_c_l(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.registers.l;
}

fn load_a_mem_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.memory.borrow().get(core.registers.get_hl());
}

fn load_b_mem_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.memory.borrow().get(core.registers.get_hl());
}

fn load_c_mem_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.memory.borrow().get(core.registers.get_hl());
}

fn load_d_mem_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.memory.borrow().get(core.registers.get_hl());
}

fn load_e_mem_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.memory.borrow().get(core.registers.get_hl());
}

fn load_h_mem_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.memory.borrow().get(core.registers.get_hl());
}

fn load_l_mem_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.memory.borrow().get(core.registers.get_hl());
}

fn load_b_a(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.registers.a;
}

fn load_b_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.registers.b;
}

fn load_b_c(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.registers.c;
}

fn load_b_d(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.registers.d;
}

fn load_b_e(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.registers.e;
}

fn load_b_h(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.registers.h;
}

fn load_b_l(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.registers.l;
}

fn nop(_: &mut Core, _: Option<Operand>) {}

fn stop(_: &mut Core, _: Option<Operand>) {}

fn halt(core: &mut Core, _: Option<Operand>) {
    core.halted = true;
}

fn load_bc_d16(core: &mut Core, operand: Option<Operand>) {
    core.registers.set_bc(operand.unwrap().word);
}

fn load_sp_d16(core: &mut Core, operand: Option<Operand>) {
    core.registers.sp = operand.unwrap().word;
}

fn load_hl_d16(core: &mut Core, operand: Option<Operand>) {
    core.registers.set_hl(operand.unwrap().word);
}

fn load_sp_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.sp = core.registers.get_hl();
}

fn add_hl_hl(core: &mut Core, _: Option<Operand>) {
    core.registers.set_hl(core.registers.get_hl());
}

fn add_hl_sp(core: &mut Core, _: Option<Operand>) {
    core.registers.set_hl(core.registers.sp);
}

fn load_de_d16(core: &mut Core, operand: Option<Operand>) {
    core.registers.set_de(operand.unwrap().word);
}

fn load_mem_bc_a(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_bc(), core.registers.a);
}

fn load_mem_hl_b(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_hl(), core.registers.b);
}

fn load_mem_hl_c(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_hl(), core.registers.c);
}

fn load_mem_hl_d(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_hl(), core.registers.d);
}

fn load_mem_hl_e(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_hl(), core.registers.e);
}

fn load_mem_hl_h(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_hl(), core.registers.h);
}

fn load_mem_hl_l(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_hl(), core.registers.l);
}

fn load_mem_hl_a(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_hl(), core.registers.a);
}

fn load_mem_de_a(core: &mut Core, _: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_de(), core.registers.a);
}

fn increment_bc(core: &mut Core, _: Option<Operand>) {
    core.registers
        .set_bc(core.registers.get_bc().wrapping_add(1));
}

fn increment_sp(core: &mut Core, _: Option<Operand>) {
    core.registers.sp = core.registers.sp.wrapping_add(1);
}

fn increment_hl(core: &mut Core, _: Option<Operand>) {
    core.registers
        .set_hl(core.registers.get_hl().wrapping_add(1));
}

fn increment_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let hl_value = core.memory.borrow().get(address);
    let results = core.alu_inc(hl_value);
    core.memory.borrow_mut().set(address, results);
}

fn decrement_hl(core: &mut Core, _: Option<Operand>) {
    core.registers
        .set_hl(core.registers.get_hl().wrapping_sub(1));
}

fn decrement_mem_hl(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    let hl_value = core.memory.borrow().get(address);
    let results = core.alu_dec(hl_value);
    core.memory.borrow_mut().set(address, results);
}

fn increment_de(core: &mut Core, _: Option<Operand>) {
    core.registers
        .set_de(core.registers.get_de().wrapping_add(1));
}

fn decrement_bc(core: &mut Core, _: Option<Operand>) {
    core.registers
        .set_bc(core.registers.get_bc().wrapping_sub(1));
}

fn decrement_de(core: &mut Core, _: Option<Operand>) {
    core.registers
        .set_de(core.registers.get_de().wrapping_sub(1));
}

fn decrement_sp(core: &mut Core, _: Option<Operand>) {
    core.registers.sp = core.registers.sp.wrapping_sub(1);
}

fn increment_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_inc(core.registers.a);
}

fn increment_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_inc(core.registers.b);
}

fn increment_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_inc(core.registers.d);
}

fn decrement_d(core: &mut Core, _: Option<Operand>) {
    core.registers.d = core.alu_dec(core.registers.d);
}

fn decrement_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_dec(core.registers.e);
}

fn decrement_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_dec(core.registers.h);
}

fn decrement_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_dec(core.registers.l);
}

fn increment_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_inc(core.registers.c);
}

fn increment_e(core: &mut Core, _: Option<Operand>) {
    core.registers.e = core.alu_inc(core.registers.e);
}

fn increment_h(core: &mut Core, _: Option<Operand>) {
    core.registers.h = core.alu_inc(core.registers.h);
}

fn increment_l(core: &mut Core, _: Option<Operand>) {
    core.registers.l = core.alu_inc(core.registers.l);
}

fn decrement_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_dec(core.registers.a);
}

fn decrement_b(core: &mut Core, _: Option<Operand>) {
    core.registers.b = core.alu_dec(core.registers.b);
}

fn decrement_c(core: &mut Core, _: Option<Operand>) {
    core.registers.c = core.alu_dec(core.registers.c);
}

fn load_a_d8(core: &mut Core, operand: Option<Operand>) {
    core.registers.a = operand.unwrap().byte;
}

fn load_b_d8(core: &mut Core, operand: Option<Operand>) {
    core.registers.b = operand.unwrap().byte;
}

fn load_c_d8(core: &mut Core, operand: Option<Operand>) {
    core.registers.c = operand.unwrap().byte;
}

fn load_d_d8(core: &mut Core, operand: Option<Operand>) {
    core.registers.d = operand.unwrap().byte;
}

fn load_e_d8(core: &mut Core, operand: Option<Operand>) {
    core.registers.e = operand.unwrap().byte;
}

fn load_h_d8(core: &mut Core, operand: Option<Operand>) {
    core.registers.h = operand.unwrap().byte;
}

fn load_l_d8(core: &mut Core, operand: Option<Operand>) {
    core.registers.l = operand.unwrap().byte;
}

fn rotate_left_carry_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_rlc(core.registers.a);
    core.registers.set_flag(Flag::Z, false);
}

fn rotate_right_carry_a(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_rrc(core.registers.a);
    core.registers.set_flag(Flag::Z, false);
}

fn rotate_left_a_through(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_rl(core.registers.a);
    core.registers.set_flag(Flag::Z, false);
}

fn rotate_right_a_through(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.alu_rr(core.registers.a);
    core.registers.set_flag(Flag::Z, false);
}

fn load_mem_sp(core: &mut Core, operand: Option<Operand>) {
    core.memory
        .borrow_mut()
        .set_word(operand.unwrap().word, core.registers.sp);
}

fn add_hl_bc(core: &mut Core, _: Option<Operand>) {
    core.alu_add_hl(core.registers.get_bc());
}

fn add_hl_de(core: &mut Core, _: Option<Operand>) {
    core.alu_add_hl(core.registers.get_de());
}

fn load_mem_hlp_a(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    core.memory.borrow_mut().set(address, core.registers.a);
    core.registers.set_hl(address + 1);
}

fn load_mem_hlm_a(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    core.memory.borrow_mut().set(address, core.registers.a);
    core.registers.set_hl(address - 1);
}

fn load_mem_hl_d8(core: &mut Core, operand: Option<Operand>) {
    let address = core.registers.get_hl();
    core.memory.borrow_mut().set(address, operand.unwrap().byte);
}

fn load_a_hlp_mem(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    core.registers.a = core.memory.borrow().get(address);
    core.registers.set_hl(address + 1);
}

fn load_a_hlm_mem(core: &mut Core, _: Option<Operand>) {
    let address = core.registers.get_hl();
    core.registers.a = core.memory.borrow().get(address);
    core.registers.set_hl(address - 1);
}

fn load_a_mem_bc(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.memory.borrow().get(core.registers.get_bc());
}

fn load_a_mem_de(core: &mut Core, _: Option<Operand>) {
    core.registers.a = core.memory.borrow().get(core.registers.get_de());
}

fn jr_r8(core: &mut Core, operand: Option<Operand>) {
    core.alu_jr(operand.unwrap().byte);
}

fn jr_nz_r8(core: &mut Core, operand: Option<Operand>) {
    if !core.registers.get_flag(Flag::Z) {
        core.alu_jr(operand.unwrap().byte);
    }
}

fn jr_nc_r8(core: &mut Core, operand: Option<Operand>) {
    if !core.registers.get_flag(Flag::C) {
        core.alu_jr(operand.unwrap().byte);
    }
}

fn jr_c_r8(core: &mut Core, operand: Option<Operand>) {
    if core.registers.get_flag(Flag::C) {
        core.alu_jr(operand.unwrap().byte);
    }
}

fn jr_z_r8(core: &mut Core, operand: Option<Operand>) {
    if core.registers.get_flag(Flag::Z) {
        core.alu_jr(operand.unwrap().byte);
    }
}

fn decimal_adjust_a(core: &mut Core, _: Option<Operand>) {
    core.alu_daa();
}

fn complement_a(core: &mut Core, _: Option<Operand>) {
    core.alu_cpl();
}

fn set_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_scf();
}

fn complement_carry(core: &mut Core, _: Option<Operand>) {
    core.alu_ccf();
}
