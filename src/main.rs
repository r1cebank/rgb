pub mod cpu;
pub mod memory;

fn main() {
    // let mut test = cpu::CPU::new();
    // test.execute(cpu::instruction::Instruction::ADD(
    //     cpu::instruction::ArithmeticTarget::HLI,
    // ));
    println!("Hello, world!");
    println!(
        "{:?}",
        cpu::instruction::Instruction::from_byte_not_prefixed(0x8f)
    );
}
