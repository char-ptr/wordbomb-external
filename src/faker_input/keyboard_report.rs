use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    A = 0x04,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Number1 = 0x1E,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    Number0,

    Enter = 0x28,
    Escape = 0x29,
    Backspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equals = 0x2E,
    LeftBracket = 0x2F,
    RightBracket = 0x30,
    Backslash = 0x31,
    NonUSHash = 0x32,
    Semicolon = 0x33,
    Apostrophe = 0x34,
    Grave = 0x35,
    Comma = 0x36,
    Period = 0x37,
    Slash = 0x38,
    CapsLock = 0x39,

    F1 = 0x3A,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    F13 = 0x68,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    RightArrow = 0x4F,  // is "Arrow" needed?
    LeftArrow = 0x50,
    DownArrow = 0x51,
    UpArrow = 0x52,
    NumLock = 0x53,
    KeypadDivide = 0x54,
    KeypadMultiply = 0x55,
    KeypadSubtract = 0x56,
    KeypadAdd = 0x57,
    KeypadEnter = 0x58,
    Keypad1 = 0x59,
    Keypad2 = 0x5A,
    Keypad3 = 0x5B,
    Keypad4 = 0x5C,
    Keypad5 = 0x5D,
    Keypad6 = 0x5E,
    Keypad7 = 0x5F,
    Keypad8 = 0x60,
    Keypad9 = 0x61,
    Keypad0 = 0x62,
    KeypadDecimal = 0x63,
    KeypadSeparator = 0x64,
    KeypadApplication = 0x65
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum KBDModifier {
    LControl = 1,
    LShift = 2,
    LAlt = 4,
    LWin = 8,
    RControl = 16,
    RShift = 32,
    RAlt = 64,
    RWin = 128
}

pub struct KBDReport {
    pub modifiers: HashSet<KBDModifier>,
    pub pressed_keys: HashSet<KeyboardKey>,
}

impl KBDReport {
    const KBD_KEY_CODES :u8 = 6u8;
    pub fn new() -> KBDReport {
        KBDReport {
            modifiers: HashSet::new(),
            pressed_keys: HashSet::new(),
        }
    }
    pub fn get_raw_shift_key_flags(&self) -> u8 {
        let mut shift_keys = 0u8;
        for modif in &self.modifiers {
            shift_keys |= *modif as u8;
        }
        shift_keys
    }
    pub fn get_raw_key_codes(&self) -> [u8; KBDReport::KBD_KEY_CODES as usize] {
        let mut key_codes = [0u8;KBDReport::KBD_KEY_CODES as usize];
        for (i,key) in self.pressed_keys.iter().enumerate() {
            if i < 6 {
                key_codes[i] = *key as u8;
            }
        }
        key_codes
    }

    pub fn key_down(mut self, key: KeyboardKey) -> Self {
        self.pressed_keys.insert(key);
        self
    }
    pub fn key_up(mut self, key: KeyboardKey) -> Self {
        self.pressed_keys.remove(&key);
        self
    }
    pub fn modifier_down(mut self, modifier: KBDModifier) -> Self {
        self.modifiers.insert(modifier);
        self
    }
    pub fn modifier_up(mut self, modifier: KBDModifier) -> Self {
        self.modifiers.remove(&modifier);
        self
    }

}