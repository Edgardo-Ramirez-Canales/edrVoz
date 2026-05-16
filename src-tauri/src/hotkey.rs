use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::Emitter;

const VK_J: u32 = b'J' as u32;

static WIN_PRESSED: AtomicBool = AtomicBool::new(false);
static RECORDING: AtomicBool = AtomicBool::new(false);
static APP_HANDLE: Mutex<Option<Arc<AppHandle>>> = Mutex::new(None);

pub fn setup_hotkey(app: AppHandle) -> Result<(), String> {
    *APP_HANDLE.lock().unwrap() = Some(Arc::new(app));

    std::thread::spawn(|| {
        unsafe {
            let hook = SetWindowsHookExA(
                WH_KEYBOARD_LL,
                Some(keyboard_hook),
                HINSTANCE(std::ptr::null_mut()),
                0,
            );

            let hook = match hook {
                Ok(h) => h,
                Err(_) => {
                    eprintln!("Failed to set hook");
                    return;
                }
            };

            let mut msg = std::mem::zeroed();
            while GetMessageA(&mut msg, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }

            let _ = UnhookWindowsHookEx(hook);
        }
    });

    Ok(())
}

unsafe extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let kbd_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
    let is_key_down = wparam.0 == WM_KEYDOWN as usize;

    match kbd_struct.vkCode {
        v if v == VK_LWIN.0 as u32 || v == VK_RWIN.0 as u32 => {
            WIN_PRESSED.store(is_key_down, Ordering::SeqCst);
        }
        VK_J => {
            if is_key_down && WIN_PRESSED.load(Ordering::SeqCst) {
                if !RECORDING.swap(true, Ordering::SeqCst) {
                    if let Ok(guard) = APP_HANDLE.lock() {
                        if let Some(app) = guard.as_ref() {
                            let _ = app.emit("recording-started", ());
                        }
                    }
                    return LRESULT(1);
                }
            } else if !is_key_down {
                if RECORDING.load(Ordering::SeqCst) {
                    RECORDING.store(false, Ordering::SeqCst);
                    if let Ok(guard) = APP_HANDLE.lock() {
                        if let Some(app) = guard.as_ref() {
                            let _ = app.emit("recording-stopped", ());
                        }
                    }
                    return LRESULT(1);
                }
            }
        }
        _ => {}
    }

    CallNextHookEx(None, code, wparam, lparam)
}
