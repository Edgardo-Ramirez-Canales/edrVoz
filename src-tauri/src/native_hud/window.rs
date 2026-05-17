use std::ptr::null;
use windows_sys::Win32::Foundation::{GetLastError, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, GetSystemMetrics, RegisterClassW, SM_CXSCREEN, SM_CYSCREEN,
    WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};

use super::{scaled, wide, NativeHud, LOGICAL_H, LOGICAL_W};

impl NativeHud {
    pub(super) fn new(scale: f64) -> Option<Self> {
        unsafe {
            let class_name = wide("EDR_VOZ_NATIVE_HUD");
            let instance = GetModuleHandleW(null());
            let wnd_class = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: instance,
                hIcon: std::ptr::null_mut(),
                hCursor: std::ptr::null_mut(),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: null(),
                lpszClassName: class_name.as_ptr(),
            };
            RegisterClassW(&wnd_class);

            let width = scaled(LOGICAL_W, scale);
            let height = scaled(LOGICAL_H, scale);
            let x = (GetSystemMetrics(SM_CXSCREEN) - width) / 2;
            let y = GetSystemMetrics(SM_CYSCREEN) - height - scaled(80, scale);

            let hwnd = CreateWindowExW(
                WS_EX_LAYERED | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST,
                class_name.as_ptr(),
                wide("EDR Voz HUD").as_ptr(),
                WS_POPUP,
                x,
                y,
                width,
                height,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                instance,
                std::ptr::null_mut(),
            );

            if hwnd.is_null() {
                log::warn!("CreateWindowExW falló para HUD nativo: {}", GetLastError());
                None
            } else {
                Some(Self {
                    hwnd: hwnd as isize,
                    scale,
                })
            }
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    DefWindowProcW(hwnd, msg, wparam, lparam)
}
