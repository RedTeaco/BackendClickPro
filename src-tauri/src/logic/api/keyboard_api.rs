use std::thread;
use std::time::Duration;
use tauri::utils::assets::phf;
use tauri::utils::assets::phf::phf_map;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{PostMessageW, WM_KEYDOWN, WM_KEYUP};
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use crate::logic::utils::window_capture::handle_selected_window;

static KEY_MAP: phf::Map<&'static str, u16> = phf_map! {
    // ========== 功能键 F1-F24 ==========
    "F1" => VK_F1.0,
    "F2" => VK_F2.0,
    "F3" => VK_F3.0,
    "F4" => VK_F4.0,
    "F5" => VK_F5.0,
    "F6" => VK_F6.0,
    "F7" => VK_F7.0,
    "F8" => VK_F8.0,
    "F9" => VK_F9.0,
    "F10" => VK_F10.0,
    "F11" => VK_F11.0,
    "F12" => VK_F12.0,
    "F13" => VK_F13.0,
    "F14" => VK_F14.0,
    "F15" => VK_F15.0,
    "F16" => VK_F16.0,
    "F17" => VK_F17.0,
    "F18" => VK_F18.0,
    "F19" => VK_F19.0,
    "F20" => VK_F20.0,
    "F21" => VK_F21.0,
    "F22" => VK_F22.0,
    "F23" => VK_F23.0,
    "F24" => VK_F24.0,

    // ========== 控制键 ==========
    "Backspace" => VK_BACK.0,
    "Tab" => VK_TAB.0,
    "Enter" => VK_RETURN.0,
    "ShiftLeft" => VK_LSHIFT.0,
    "ShiftRight" => VK_RSHIFT.0,
    "ControlLeft" => VK_LCONTROL.0,
    "ControlRight" => VK_RCONTROL.0,
    "AltLeft" => VK_LMENU.0,
    "AltRight" => VK_RMENU.0,
    "Pause" => VK_PAUSE.0,
    "CapsLock" => VK_CAPITAL.0,
    "Escape" => VK_ESCAPE.0,
    "Space" => VK_SPACE.0,
    "PageUp" => VK_PRIOR.0,
    "PageDown" => VK_NEXT.0,
    "End" => VK_END.0,
    "Home" => VK_HOME.0,
    "ArrowLeft" => VK_LEFT.0,
    "ArrowUp" => VK_UP.0,
    "ArrowRight" => VK_RIGHT.0,
    "ArrowDown" => VK_DOWN.0,
    "PrintScreen" => VK_SNAPSHOT.0,
    "Insert" => VK_INSERT.0,
    "Delete" => VK_DELETE.0,
    "ScrollLock" => VK_SCROLL.0,
    "NumLock" => VK_NUMLOCK.0,
    "ContextMenu" => VK_APPS.0,

    // ========== 数字小键盘 ==========
    "Numpad0" => VK_NUMPAD0.0,
    "Numpad1" => VK_NUMPAD1.0,
    "Numpad2" => VK_NUMPAD2.0,
    "Numpad3" => VK_NUMPAD3.0,
    "Numpad4" => VK_NUMPAD4.0,
    "Numpad5" => VK_NUMPAD5.0,
    "Numpad6" => VK_NUMPAD6.0,
    "Numpad7" => VK_NUMPAD7.0,
    "Numpad8" => VK_NUMPAD8.0,
    "Numpad9" => VK_NUMPAD9.0,
    "NumpadMultiply" => VK_MULTIPLY.0,
    "NumpadAdd" => VK_ADD.0,
    "NumpadSubtract" => VK_SUBTRACT.0,
    "NumpadDecimal" => VK_DECIMAL.0,
    "NumpadDivide" => VK_DIVIDE.0,
    "NumpadEnter" => VK_RETURN.0,  // 小键盘回车与主键盘相同

    // ========== 主键盘符号（美式布局） ==========
    "Backquote" => VK_OEM_3.0,
    "Minus" => VK_OEM_MINUS.0,
    "Equal" => VK_OEM_PLUS.0,
    "BracketLeft" => VK_OEM_4.0,
    "BracketRight" => VK_OEM_6.0,
    "Backslash" => VK_OEM_5.0,
    "Semicolon" => VK_OEM_1.0,
    "Quote" => VK_OEM_7.0,
    "Comma" => VK_OEM_COMMA.0,
    "Period" => VK_OEM_PERIOD.0,
    "Slash" => VK_OEM_2.0,

    // ========== 多媒体键 ==========
    "VolumeMute" => VK_VOLUME_MUTE.0,
    "VolumeDown" => VK_VOLUME_DOWN.0,
    "VolumeUp" => VK_VOLUME_UP.0,
    "MediaPlayPause" => VK_MEDIA_PLAY_PAUSE.0,
    "MediaStop" => VK_MEDIA_STOP.0,
    "MediaTrackNext" => VK_MEDIA_NEXT_TRACK.0,
    "MediaTrackPrevious" => VK_MEDIA_PREV_TRACK.0,
    "BrowserHome" => VK_BROWSER_HOME.0,
    "BrowserSearch" => VK_BROWSER_SEARCH.0,
    "BrowserFavorites" => VK_BROWSER_FAVORITES.0,
    "BrowserBack" => VK_BROWSER_BACK.0,
    "BrowserForward" => VK_BROWSER_FORWARD.0,
    "BrowserRefresh" => VK_BROWSER_REFRESH.0,
    "BrowserStop" => VK_BROWSER_STOP.0,
    "LaunchMail" => VK_LAUNCH_MAIL.0,
    "LaunchApp1" => VK_LAUNCH_APP1.0,
    "LaunchApp2" => VK_LAUNCH_APP2.0,
};

fn code_to_vk(code: &str) -> Option<u16> {
    if let Some(&vk) = KEY_MAP.get(code) {
        return Some(vk);
    }
        // 字母键: "KeyA" -> VK_A
        if code.starts_with("Key") && code.len() == 4 {
            let letter = code.chars().nth(3)?;
            if letter.is_ascii_uppercase() {
                return Some(letter as u16);
            }
        }
        // 数字键 "Digit1"  -> VK_1
        if code.starts_with("Digit") && code.len() == 6 {
            let digit_char = code.chars().nth(5)?;
            if digit_char.is_ascii_digit() {
                return Some(digit_char as u16)
            }
        }
        None
}

fn get_scan_code(vk: u16) -> u32 {
    unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC)}
}

pub(crate) fn keyboard_down(hwnd_u: usize, key_code: &str) -> windows::core::Result<()> {
    let hwnd = handle_selected_window(hwnd_u).unwrap();
    if hwnd.0 == std::ptr::null_mut() {
        eprintln!("Error: Invalid window handle");
        return Ok(());
    }
    println!("[DEBUG][KEYBOARD] Event action: key {} down", key_code);
    let vk = code_to_vk(key_code);
    let scan_code = get_scan_code(vk.unwrap());
    let lparam = ((scan_code) << 16) | 1;
    unsafe {
        PostMessageW(hwnd.into(), WM_KEYDOWN, WPARAM(vk.unwrap() as usize), LPARAM(lparam as isize)).expect("failed to post keydown message");
    }
    Ok(())
}

pub(crate) fn keyboard_up(hwnd_u: usize, key_code: &str) -> windows::core::Result<()> {
    let hwnd = handle_selected_window(hwnd_u).unwrap();
    if hwnd.0 == std::ptr::null_mut() {
        eprintln!("Error: Invalid window handle");
        return Ok(());
    }
    println!("[DEBUG][KEYBOARD] Event action: key {} up", key_code);
    let vk = code_to_vk(key_code);
    let scan_code = get_scan_code(vk.unwrap());
    let lparam = ((scan_code) << 16) | 0xC0000001;
    unsafe {
        PostMessageW(hwnd.into(), WM_KEYUP, WPARAM(vk.unwrap() as usize), LPARAM(lparam as isize)).expect("failed to post keyup message");
    }
    Ok(())
}

pub fn keyboard_click(hwnd: usize, key: &str) -> windows::core::Result<()> {
    keyboard_down(hwnd, key)?;
    thread::sleep(Duration::from_millis(50));
    keyboard_up(hwnd, key)?;
    Ok(())
}