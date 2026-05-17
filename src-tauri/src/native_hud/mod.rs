#![cfg(target_os = "windows")]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};
use std::time::Duration;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOWNOACTIVATE};

mod primitives;
mod renderer;
mod window;

const LOGICAL_W: i32 = 280;
const LOGICAL_H: i32 = 50;
const CONTROL_H: i32 = 30;
const CENTER_W: i32 = 88;
const BUTTON_R: i32 = 12;
const SUPER_SAMPLE: i32 = 3;

#[derive(Clone, Copy)]
enum HudState {
    Recording,
    Transcribing,
    Ready,
    Error,
}

struct NativeHud {
    hwnd: isize,
    scale: f64,
}

static HUD: LazyLock<Mutex<Option<NativeHud>>> = LazyLock::new(|| Mutex::new(None));
static RECORDING_ANIMATION: AtomicBool = AtomicBool::new(false);

pub fn init(scale: f64) {
    let mut hud = HUD.lock().unwrap();
    if hud.is_none() {
        match NativeHud::new(scale) {
            Some(native) => {
                log::info!("HUD nativo Win32 creado correctamente");
                *hud = Some(native);
            }
            None => log::warn!("No se pudo crear el HUD nativo Win32"),
        }
    }
}

pub fn show_recording() -> bool {
    if RECORDING_ANIMATION
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        std::thread::spawn(|| {
            let mut frame = 0_u32;
            while RECORDING_ANIMATION.load(Ordering::SeqCst) {
                show_frame(HudState::Recording, frame);
                frame = frame.wrapping_add(1);
                std::thread::sleep(Duration::from_millis(80));
            }
        });
    }
    show_frame(HudState::Recording, 0)
}

pub fn show_transcribing() -> bool {
    RECORDING_ANIMATION.store(false, Ordering::SeqCst);
    show_frame(HudState::Transcribing, 0)
}

pub fn show_ready() -> bool {
    RECORDING_ANIMATION.store(false, Ordering::SeqCst);
    show_frame(HudState::Ready, 0)
}

pub fn show_error() -> bool {
    RECORDING_ANIMATION.store(false, Ordering::SeqCst);
    show_frame(HudState::Error, 0)
}

pub fn hide() {
    RECORDING_ANIMATION.store(false, Ordering::SeqCst);
    if let Some(hud) = HUD.lock().unwrap().as_ref() {
        unsafe {
            ShowWindow(hud.hwnd as HWND, SW_HIDE);
        }
    }
}

fn show_frame(state: HudState, frame: u32) -> bool {
    if let Some(hud) = HUD.lock().unwrap().as_ref() {
        if !hud.render(state, frame) {
            return false;
        }
        unsafe {
            ShowWindow(hud.hwnd as HWND, SW_SHOWNOACTIVATE);
        }
        true
    } else {
        log::warn!("HUD nativo solicitado antes de inicializarse");
        false
    }
}

fn scaled(value: i32, scale: f64) -> i32 {
    ((value as f64) * scale).round() as i32
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
