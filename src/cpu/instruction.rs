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

    (instruction_set, cb_instruction_set)
}

fn nop(_: &mut Core, _: Option<Operand>) {}

fn stop(_: &mut Core, _: Option<Operand>) {}

fn load_bc_d16(core: &mut Core, operand: Option<Operand>) {
    core.registers.set_bc(operand.unwrap().word);
}

fn load_sp_d16(core: &mut Core, operand: Option<Operand>) {
    core.registers.sp = operand.unwrap().word;
}

fn load_hl_d16(core: &mut Core, operand: Option<Operand>) {
    core.registers.set_hl(operand.unwrap().word);
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
