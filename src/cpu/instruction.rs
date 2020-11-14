use super::sm80::Core;
use crate::cpu::registers::Flag;
use std::collections::HashMap;

pub struct Operand {
    pub byte: u8,
    pub word: u16,
}

pub struct Instruction {
    pub name: &'static str,
    pub opcode: u8,
    pub operand_length: u8,
    pub cycles: u8,
    pub exec: Box<dyn Fn(&mut Core, Operand)>,
}

impl Instruction {
    pub fn new(
        name: &'static str,
        opcode: u8,
        operand_length: u8,
        cycles: u8,
        exec: Box<dyn Fn(&mut Core, Operand)>,
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
        Instruction::new("INC DE", 0x13, 0, 8, Box::new(increment_de)),
    );

    (instruction_set, cb_instruction_set)
}

fn nop(_: &mut Core, _: Operand) {}

fn stop(_: &mut Core, _: Operand) {}

fn load_bc_d16(core: &mut Core, operand: Operand) {
    core.registers.set_bc(operand.word);
}

fn load_de_d16(core: &mut Core, operand: Operand) {
    core.registers.set_de(operand.word);
}

fn load_mem_bc_a(core: &mut Core, _: Operand) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_bc(), core.registers.a);
}

fn load_mem_de_a(core: &mut Core, _: Operand) {
    core.memory
        .borrow_mut()
        .set(core.registers.get_de(), core.registers.a);
}

fn increment_bc(core: &mut Core, _: Operand) {
    core.registers
        .set_bc(core.registers.get_bc().wrapping_add(1));
}

fn increment_de(core: &mut Core, _: Operand) {
    core.registers
        .set_de(core.registers.get_de().wrapping_add(1));
}

fn decrement_bc(core: &mut Core, _: Operand) {
    core.registers
        .set_bc(core.registers.get_bc().wrapping_sub(1));
}

fn increment_b(core: &mut Core, _: Operand) {
    core.registers.b = core.alu_inc(core.registers.b);
}

fn increment_c(core: &mut Core, _: Operand) {
    core.registers.c = core.alu_inc(core.registers.c);
}

fn decrement_b(core: &mut Core, _: Operand) {
    core.registers.b = core.alu_dec(core.registers.b);
}

fn decrement_c(core: &mut Core, _: Operand) {
    core.registers.c = core.alu_dec(core.registers.c);
}

fn load_b_d8(core: &mut Core, operand: Operand) {
    core.registers.b = operand.byte;
}

fn load_c_d8(core: &mut Core, operand: Operand) {
    core.registers.c = operand.byte;
}

fn rotate_left_carry_a(core: &mut Core, _: Operand) {
    core.registers.a = core.alu_rlc(core.registers.a);
    core.registers.set_flag(Flag::Z, false);
}

fn rotate_right_carry_a(core: &mut Core, _: Operand) {
    core.registers.a = core.alu_rrc(core.registers.a);
    core.registers.set_flag(Flag::Z, false);
}

fn load_mem_sp(core: &mut Core, operand: Operand) {
    core.memory
        .borrow_mut()
        .set_word(operand.word, core.registers.sp);
}

fn add_hl_bc(core: &mut Core, _: Operand) {
    core.alu_add_hl(core.registers.get_bc());
}

fn load_a_mem_bc(core: &mut Core, _: Operand) {
    core.registers.a = core.memory.borrow().get(core.registers.get_bc());
}
