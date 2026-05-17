use std::ffi::c_void;
use windows_sys::Win32::Foundation::{GetLastError, HWND, POINT, SIZE};
use windows_sys::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject, SelectObject, AC_SRC_ALPHA,
    AC_SRC_OVER, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, BLENDFUNCTION, DIB_RGB_COLORS,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, UpdateLayeredWindow, SM_CXSCREEN, SM_CYSCREEN, ULW_ALPHA,
};

use super::primitives::{
    draw_button_check, draw_button_shell, draw_button_x, draw_control_body, draw_control_shadow,
    draw_error_mark, draw_ready_mark, draw_wave,
};
use super::{scaled, HudState, NativeHud, BUTTON_R, CENTER_W, CONTROL_H, LOGICAL_H, LOGICAL_W};

impl NativeHud {
    pub(super) fn render(&self, state: HudState, frame: u32) -> bool {
        let width = scaled(LOGICAL_W, self.scale);
        let height = scaled(LOGICAL_H, self.scale);
        let pixels = render_pixels(width, height, self.scale, state, frame);

        unsafe {
            let screen_dc = windows_sys::Win32::Graphics::Gdi::GetDC(std::ptr::null_mut());
            if screen_dc.is_null() {
                log::warn!("GetDC falló para HUD nativo: {}", GetLastError());
                return false;
            }

            let mem_dc = CreateCompatibleDC(screen_dc);
            if mem_dc.is_null() {
                log::warn!(
                    "CreateCompatibleDC falló para HUD nativo: {}",
                    GetLastError()
                );
                windows_sys::Win32::Graphics::Gdi::ReleaseDC(std::ptr::null_mut(), screen_dc);
                return false;
            }

            let mut bits: *mut c_void = std::ptr::null_mut();
            let info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [Default::default(); 1],
            };

            let bitmap = CreateDIBSection(
                mem_dc,
                &info,
                DIB_RGB_COLORS,
                &mut bits,
                std::ptr::null_mut(),
                0,
            );

            if bitmap.is_null() || bits.is_null() {
                log::warn!("CreateDIBSection falló para HUD nativo: {}", GetLastError());
                DeleteDC(mem_dc);
                windows_sys::Win32::Graphics::Gdi::ReleaseDC(std::ptr::null_mut(), screen_dc);
                return false;
            }

            std::ptr::copy_nonoverlapping(pixels.as_ptr(), bits as *mut u8, pixels.len());
            let old = SelectObject(mem_dc, bitmap);

            let screen_w = GetSystemMetrics(SM_CXSCREEN);
            let screen_h = GetSystemMetrics(SM_CYSCREEN);
            let dst = POINT {
                x: (screen_w - width) / 2,
                y: screen_h - height - scaled(80, self.scale),
            };
            let size = SIZE {
                cx: width,
                cy: height,
            };
            let src = POINT { x: 0, y: 0 };
            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };

            let updated = UpdateLayeredWindow(
                self.hwnd as HWND,
                screen_dc,
                &dst,
                &size,
                mem_dc,
                &src,
                0,
                &blend,
                ULW_ALPHA,
            );
            if updated == 0 {
                log::warn!(
                    "UpdateLayeredWindow falló para HUD nativo: {}",
                    GetLastError()
                );
            }

            SelectObject(mem_dc, old);
            DeleteObject(bitmap);
            DeleteDC(mem_dc);
            windows_sys::Win32::Graphics::Gdi::ReleaseDC(std::ptr::null_mut(), screen_dc);
            updated != 0
        }
    }
}

fn render_pixels(width: i32, height: i32, scale: f64, state: HudState, frame: u32) -> Vec<u8> {
    let mut pixels = vec![0_u8; (width * height * 4) as usize];
    let center_y = scaled(LOGICAL_H / 2, scale);

    let left_x = scaled(62, scale);
    let center_x = scaled((LOGICAL_W - CENTER_W) / 2, scale);
    let center_w = scaled(CENTER_W, scale);
    let control_h = scaled(CONTROL_H, scale);
    let center_y_top = center_y - control_h / 2;
    let right_x = scaled(218, scale);
    let button_r = scaled(BUTTON_R, scale);

    draw_button_shell(&mut pixels, width, height, left_x, center_y, button_r);
    draw_control_shadow(
        &mut pixels,
        width,
        height,
        center_x,
        center_y_top,
        center_w,
        control_h,
        control_h as f32 / 2.0,
    );
    draw_control_body(
        &mut pixels,
        width,
        height,
        center_x,
        center_y_top,
        center_w,
        control_h,
        control_h as f32 / 2.0,
    );
    draw_button_shell(&mut pixels, width, height, right_x, center_y, button_r);

    draw_button_x(&mut pixels, width, height, left_x, center_y, scale);
    draw_button_check(
        &mut pixels,
        width,
        height,
        right_x,
        center_y,
        scale,
        matches!(state, HudState::Ready),
    );

    match state {
        HudState::Recording => draw_wave(
            &mut pixels,
            width,
            height,
            center_x,
            center_w,
            scale,
            255,
            frame,
        ),
        HudState::Transcribing => draw_wave(
            &mut pixels,
            width,
            height,
            center_x,
            center_w,
            scale,
            150,
            0,
        ),
        HudState::Ready => draw_ready_mark(&mut pixels, width, height, scale),
        HudState::Error => draw_error_mark(&mut pixels, width, height, scale),
    }

    pixels
}
