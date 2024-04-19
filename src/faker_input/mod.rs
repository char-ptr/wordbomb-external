pub mod keyboard_report;
pub mod mouse_report;
use std::ffi::c_void;

use self::{keyboard_report::KBDReport, mouse_report::MouseReport};

type HNDL = *mut c_void;

#[link(name = "FakerInputDll", kind = "dylib")]
extern "C" {
    pub fn fakerinput_alloc() -> *mut c_void;
    pub fn fakerinput_free(vmulti: HNDL) -> c_void;
    pub fn fakerinput_connect(vmulti: HNDL) -> bool;
    pub fn fakerinput_disconnect(vmulti: HNDL) -> bool;
    pub fn fakerinput_update_keyboard(vmulti: HNDL, shiftKeyFlags: u8, keyCodes: *const u8)
        -> bool;
    pub fn fakerinput_update_keyboard_enhanced(
        vmulti: HNDL,
        mediaKeys: u8,
        enhancedKeys: u8,
    ) -> bool;
    pub fn fakerinput_update_relative_mouse(
        vmulti: HNDL,
        button: u8,
        x: i16,
        y: i16,
        wheelPosition: u8,
        hWheelPosition: u8,
    ) -> bool;
    pub fn fakerinput_update_absolute_mouse(
        vmulti: HNDL,
        button: u8,
        x: u16,
        y: u16,
        wheelPosition: u8,
        hWheelPosition: u8,
    ) -> bool;
}

pub struct FakerInput {
    vmulti: HNDL,
    connected: bool,
}

impl FakerInput {
    pub fn new() -> Self {
        let vmulti = unsafe { fakerinput_alloc() };
        Self {
            vmulti,
            connected: false,
        }
    }

    pub fn connect(&mut self) -> bool {
        if self.connected {
            return true;
        }
        self.connected = unsafe { fakerinput_connect(self.vmulti) };
        self.connected
    }

    pub fn disconnect(&mut self) -> bool {
        if !self.connected {
            return true;
        }
        self.connected = !unsafe { fakerinput_disconnect(self.vmulti) };
        !self.connected
    }

    pub fn update_keyboard(&mut self, report: KBDReport) -> bool {
        if !self.connected {
            return false;
        }
        let codes = report.get_raw_key_codes();
        unsafe {
            fakerinput_update_keyboard(
                self.vmulti,
                report.get_raw_shift_key_flags(),
                codes.as_ptr(),
            )
        }
    }

    // pub fn update_keyboard_enhanced(&mut self, mediaKeys: u8, enhancedKeys: u8) -> bool {
    //     if !self.connected {
    //         return false;
    //     }
    //     unsafe { fakerinput_update_keyboard_enhanced(self.vmulti, mediaKeys, enhancedKeys) }
    // }

    pub fn update_relative_mouse(&mut self, report: &MouseReport) -> bool {
        if !self.connected {
            return false;
        }
        unsafe {
            fakerinput_update_relative_mouse(
                self.vmulti,
                report.buttons,
                report.x,
                report.y,
                report.wheel,
                report.h_wheel,
            )
        }
    }

    pub fn update_absolute_mouse(&mut self, report: &MouseReport) -> bool {
        use mouse_position::mouse_position::Mouse;
        // if !self.connected {
        //     return false;
        // }
        // println!("{:?}", report);
        // unsafe { fakerinput_update_absolute_mouse(self.vmulti, report.buttons, report.x , report.y , report.wheel, report.h_wheel) }

        // it's broken so we'll just use relative mouse for now
        let current_position = Mouse::get_mouse_position();
        if let Mouse::Position { x, y } = current_position {
            let mut new_report = report.clone();
            //convert to relative
            new_report.x = (report.x as i32 - x as i32) as i16;
            new_report.y = (report.y as i32 - y as i32) as i16;
            println!(" {report:?} -> {:?} [ {x}, {y} ]", new_report);
            self.update_relative_mouse(&new_report)
        } else {
            false
        }
    }
}

impl Drop for FakerInput {
    fn drop(&mut self) {
        self.disconnect();
        unsafe { fakerinput_free(self.vmulti) };
    }
}

