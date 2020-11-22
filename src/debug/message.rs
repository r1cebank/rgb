use super::command::DebugCommand;
use crate::cpu::registers::Registers;
use crate::ppu::Tile;

#[derive(Clone, Debug)]
pub enum DebugMessage {
    LogUpdate(String),
    DebugCommand(DebugCommand),
    MemoryUpdate(Vec<u8>),
    RegisterUpdate(Registers),
    TileUpdate(Vec<Tile>),
}
