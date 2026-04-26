use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{PostMessageW, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP};
use crate::logic::utils::window_capture::handle_selected_window;

pub const XBUTTON1: u16 = 0x0001;
pub const XBUTTON2: u16 = 0x0002;

macro_rules! MAKELPARAM {
    ($low:expr, $high:expr) => {
        ((($low & 0xffff) as u32) | (($high & 0xffff) as u32) << 16 ) as _
    };
}

// 通用底层函数
fn mouse_send_message(
    hwnd_u: usize,
    msg: u32,
    wparam: WPARAM,
    x: i32,
    y: i32,
) -> windows::core::Result<()> {
    let hwnd = handle_selected_window(hwnd_u).unwrap();
    if hwnd.0 == std::ptr::null_mut() {
        eprintln!("Error: Invalid window handle");
        return Ok(());
    }
    let lparam = LPARAM(MAKELPARAM!(x, y));
    unsafe {
        PostMessageW(Some(hwnd), msg, wparam, lparam).expect("failed to post message");
    }
    Ok(())
}


/// 执行鼠标操作
///
/// # 参数
/// * `hwnd` - 窗口句柄，指定要发送鼠标消息的目标窗口
/// * `button` - 鼠标按钮类型，可以是"left"、"right"、"middle"、"wheel"、"xbutton1"或"xbutton2"
/// * `action` - 鼠标操作类型，可以是"down"、"up"、"click"或"scroll"
/// * `x` - 可选的鼠标X坐标，默认为0
/// * `y` - 可选的鼠标Y坐标，默认为0
/// * `delta` - 可选的滚轮滚动量，默认为0
///
/// # 返回值
/// 返回一个`windows::core::Result<()>`，表示操作是否成功
pub fn mouse_action(
    hwnd: usize,
    button: &str,
    action: &str,
    x: Option<i32>,
    y: Option<i32>,
    delta: Option<i32>
) -> windows::core::Result<()> {
    // 设置默认坐标值
    let x = x.unwrap_or(0);
    let y = y.unwrap_or(0);

    // 根据按钮类型和操作类型匹配执行不同的鼠标操作
    match (button, action) {
        // 滚轮操作
        ("wheel", _ ) => {
            let delta = delta.unwrap_or(0);
            // 将滚轮偏移量转换为Windows消息格式
            let wparam = WPARAM((((delta as u32) << 16) & 0xFFFF0000) as usize);
            mouse_send_message(hwnd, WM_MOUSEWHEEL, wparam, x, y)?;
        }
        // 标准鼠标按钮操作
        (btn, act) if ["left", "right", "middle"].contains(&btn) => {
            // 根据按钮类型确定按下和释放的消息类型
            let (down_msg, up_msg) = match btn {
                "left" => (WM_LBUTTONDOWN, WM_LBUTTONUP),
                "right" => (WM_RBUTTONDOWN, WM_RBUTTONUP),
                "middle" => (WM_MBUTTONDOWN, WM_MBUTTONUP),
                _ => unreachable!(), // This should never happen
            };
            match act {
                "down" => mouse_send_message(hwnd, down_msg, WPARAM(0), x, y)?,
                "up" => mouse_send_message(hwnd, up_msg, WPARAM(0), x, y)?,
                "click" => {
                    mouse_send_message(hwnd, down_msg, WPARAM(0), x, y)?;
                    mouse_send_message(hwnd, up_msg, WPARAM(0), x, y)?;
                }
                _ => eprintln!("Unsupported action: '{}' for button '{}'", act, btn),
            }
        }
        // 侧键
        (btn, act) if btn == "xbutton1" || btn == "xbutton2" => {
            let xbutton = if btn == "xbutton1" {XBUTTON1} else {XBUTTON2};
            let wparam = WPARAM((((xbutton as u32) << 16) | 0xFFFF0000) as usize);
            match act {
                "down" => mouse_send_message(hwnd, WM_XBUTTONDOWN, wparam, x, y)?,
                "up" => mouse_send_message(hwnd, WM_XBUTTONUP, wparam, x, y)?,
                "click" => {
                    mouse_send_message(hwnd, WM_XBUTTONDOWN, wparam, x, y)?;
                    mouse_send_message(hwnd, WM_XBUTTONUP, wparam, x, y)?;
                }
                _ => eprintln!("Unsupported action: '{}' for button '{}'", act, btn),
            }
        }
        _ => eprintln!("Unsupported button '{}' or action '{}'", button, action),
    }
    Ok(())
}