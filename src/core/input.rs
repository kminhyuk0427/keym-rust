use windows::Win32::UI::Input::KeyboardAndMouse::*;
use std::mem;
use super::scancode::SCANCODE;

const KEYEVENTF_SCANCODE: u32 = 0x0008;
const KEYEVENTF_KEYUP: u32 = 0x0002;
const KEYEVENTF_EXTENDEDKEY: u32 = 0x0001;

pub struct InputSystem;

impl InputSystem {
    #[inline(always)]
    pub fn send_key(key: &str, is_keyup: bool) -> bool {
        let Some(scancode) = SCANCODE.get(key) else {
            return false;
        };
        
        let is_extended = SCANCODE.is_extended(key);
        
        unsafe {
            Self::send_input_raw(scancode, is_extended, is_keyup);
        }
        
        true
    }
    
    #[inline(always)]
    unsafe fn send_input_raw(scancode: u16, is_extended: bool, is_keyup: bool) {
        let mut flags = KEYEVENTF_SCANCODE;
        if is_extended {
            flags |= KEYEVENTF_EXTENDEDKEY;
        }
        if is_keyup {
            flags |= KEYEVENTF_KEYUP;
        }
        
        let mut input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: scancode,
                    dwFlags: KEYBD_EVENT_FLAGS(flags),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        
        SendInput(&mut [input], mem::size_of::<INPUT>() as i32);
    }
    
    #[inline(always)]
    pub fn press_key(key: &str) -> bool {
        Self::send_key(key, false)
    }
    
    #[inline(always)]
    pub fn release_key(key: &str) -> bool {
        Self::send_key(key, true)
    }
    
    pub fn tap_key(key: &str, hold_ms: u64) {
        if Self::press_key(key) {
            if hold_ms > 0 {
                spin_sleep::sleep(std::time::Duration::from_micros(hold_ms * 1000));
            }
            Self::release_key(key);
        }
    }
}