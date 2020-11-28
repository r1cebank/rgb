use super::joypad::JoyPadKey;

pub enum InputMessage {
    KeyUp(JoyPadKey),
    KeyDown(JoyPadKey),
}
