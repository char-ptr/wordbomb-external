use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButtons {
    Left = 0x01,
    Right = 0x02,
    Middle = 0x04,
    XButton1 = 0x08,
    XButton2 = 0x10,
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct MouseReport {
    pub held_buttons: HashSet<MouseButtons>,
    pub buttons: u8,
    pub x: i16,
    pub y: i16,
    pub wheel: u8,
    pub h_wheel: u8,
}
impl MouseReport {
    pub fn new() -> Self {
        Self {
            held_buttons: HashSet::new(),
            x: 0,
            y: 0,
            wheel: 0,
            h_wheel: 0,
            buttons: 0,
        }
    }
    pub fn reset_position(&mut self) {
        self.x = 0;
        self.y = 0;
        self.wheel = 0;
        self.h_wheel = 0;
    }
    pub fn button_down(&mut self, button: MouseButtons)  {
        self.held_buttons.insert(button);
        self.buttons |= button as u8;
    }
    pub fn button_up(&mut self, button: MouseButtons) {
        self.held_buttons.remove(&button);
        self.buttons &= !(button as u8);
    }
    pub fn reset(&mut self) {
        self.held_buttons.clear();
        self.reset_position();
    }
}