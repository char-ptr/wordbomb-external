pub mod keyboard_report;
pub mod mouse_report;

use std::ffi::c_void;

use libloading::{Library, Symbol};

use self::{fn_types::UpdateAbsoluteMouse, keyboard_report::KBDReport, mouse_report::MouseReport};

pub type HNDL = *mut c_void;
mod fn_types {
    use super::HNDL;
    use std::ffi::c_void;

    pub type Alloc = unsafe extern "C" fn() -> *mut c_void;
    pub type Free = unsafe extern "C" fn(vmulti: HNDL) -> c_void;
    pub type Connect = unsafe extern "C" fn(vmulti: HNDL) -> bool;
    pub type Disconnect = unsafe extern "C" fn(vmulti: HNDL) -> bool;
    pub type UpdateKeyboard =
        unsafe extern "C" fn(vmulti: HNDL, shiftKeyFlags: u8, keyCodes: *const u8) -> bool;
    pub type UpdateKeyboardEnhanced =
        unsafe extern "C" fn(vmulti: HNDL, mediaKeys: u8, enhancedKeys: u8) -> bool;
    pub type UpdateRelativeMouse = unsafe extern "C" fn(
        vmulti: HNDL,
        button: u8,
        x: i16,
        y: i16,
        wheelPosition: u8,
        hWheelPosition: u8,
    ) -> bool;
    pub type UpdateAbsoluteMouse = unsafe extern "C" fn(
        vmulti: HNDL,
        button: u8,
        x: u16,
        y: u16,
        wheelPosition: u8,
        hWheelPosition: u8,
    ) -> bool;
}

struct FakerInputTableFnMap {
    _lib: Library,
}
impl FakerInputTableFnMap {
    fn alloc(&self) -> HNDL {
        unsafe {
            self._lib
                .get::<Symbol<fn_types::Alloc>>(b"fakerinput_alloc")
                .unwrap()()
        }
    }
    fn free(&self, vmulti: HNDL) {
        unsafe {
            self._lib
                .get::<Symbol<fn_types::Free>>(b"fakerinput_free")
                .unwrap()(vmulti);
        }
    }
    fn connect(&self, vmulti: HNDL) -> bool {
        unsafe {
            self._lib
                .get::<Symbol<fn_types::Connect>>(b"fakerinput_connect")
                .unwrap()(vmulti)
        }
    }
    fn disconnect(&self, vmulti: HNDL) -> bool {
        unsafe {
            self._lib
                .get::<Symbol<fn_types::Disconnect>>(b"fakerinput_disconnect")
                .unwrap()(vmulti)
        }
    }
    fn update_keyboard(&self, vmulti: HNDL, shiftKeyFlags: u8, keyCodes: *const u8) -> bool {
        unsafe {
            self._lib
                .get::<Symbol<fn_types::UpdateKeyboard>>(b"fakerinput_update_keyboard")
                .unwrap()(vmulti, shiftKeyFlags, keyCodes)
        }
    }
    fn update_keyboard_enhanced(&self, vmulti: HNDL, mediaKeys: u8, enhancedKeys: u8) -> bool {
        unsafe {
            self._lib
                .get::<Symbol<fn_types::UpdateKeyboardEnhanced>>(
                    b"fakerinput_update_keyboard_enhanced",
                )
                .unwrap()(vmulti, mediaKeys, enhancedKeys)
        }
    }
    fn update_relative_mouse(
        &self,
        vmulti: HNDL,
        button: u8,
        x: i16,
        y: i16,
        wheelPosition: u8,
        hWheelPosition: u8,
    ) -> bool {
        unsafe {
            self._lib
                .get::<Symbol<fn_types::UpdateRelativeMouse>>(b"fakerinput_update_relative_mouse")
                .unwrap()(vmulti, button, x, y, wheelPosition, hWheelPosition)
        }
    }
    fn update_absolute_mouse(
        &self,
        vmulti: HNDL,
        button: u8,
        x: u16,
        y: u16,
        wheelPosition: u8,
        hWheelPosition: u8,
    ) -> bool {
        unsafe {
            self._lib
                .get::<Symbol<fn_types::UpdateAbsoluteMouse>>(b"fakerinput_update_absolute_mouse")
                .unwrap()(vmulti, button, x, y, wheelPosition, hWheelPosition)
        }
    }
    unsafe fn new() -> Option<Self> {
        println!("Loading library");
        let lib = Library::new("./FakerInputDll.dll").ok()?;
        Some(FakerInputTableFnMap { _lib: lib })
    }
}
pub struct FakerInput {
    vmulti: HNDL,
    connected: bool,
    map: FakerInputTableFnMap,
}

impl FakerInput {
    pub fn new() -> Option<Self> {
        let table = unsafe { FakerInputTableFnMap::new()? };
        let vmulti = table.alloc();
        Some(Self {
            vmulti,
            connected: false,
            map: table,
        })
    }

    pub fn connect(&mut self) -> bool {
        if self.connected {
            return true;
        }
        self.connected = unsafe { self.map.connect(self.vmulti) };
        self.connected
    }

    pub fn disconnect(&mut self) -> bool {
        if !self.connected {
            return true;
        }
        self.connected = !unsafe { self.map.disconnect(self.vmulti) };
        !self.connected
    }

    pub fn update_keyboard(&mut self, report: KBDReport) -> bool {
        if !self.connected {
            return false;
        }
        let codes = report.get_raw_key_codes();
        unsafe {
            self.map.update_keyboard(
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
            self.map.update_relative_mouse(
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
        unsafe { self.map.free(self.vmulti) };
    }
}
