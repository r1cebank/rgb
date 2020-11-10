use super::command::DebugCommand;
use crate::cpu::registers::Registers;

#[derive(Copy, Clone, Debug)]
pub enum DebugMessage {
    DebugCommand(DebugCommand),
    RegisterUpdate(Registers),
}
