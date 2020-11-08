use flume::Sender;

pub enum DebugMessage {}

pub trait Debuggable {
    fn peek(address: u16) -> u8;
    fn poke(address: u16, value: u8);
    fn dump() -> Vec<u8>;
    fn freeze();
    fn unfreeze();
    fn is_frozen() -> bool;
    fn break_at(address: u16, break_notification_sender: Sender<DebugMessage>);
    fn send(debug_message: DebugMessage);
}
