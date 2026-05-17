use arboard::Clipboard;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::thread;
use std::time::Duration;

// Copia el texto al portapapeles y simula Ctrl+V en la ventana activa.
// Se ejecuta en un thread separado para no bloquear el runtime async
// y para cumplir con los requisitos de COM de Windows.
pub fn paste_text(text: &str) {
    let text = text.to_string();
    thread::spawn(move || {
        if let Err(e) = try_paste(&text) {
            log::error!("Error al pegar texto en ventana activa: {e}");
        }
    });
}

fn try_paste(text: &str) -> Result<(), String> {
    // Copiar al portapapeles
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())?;

    // Espera breve para que el portapapeles esté listo antes de simular la tecla
    thread::sleep(Duration::from_millis(100));

    // Simular Ctrl+V en la ventana que tenga el foco
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    enigo.key(Key::Control, Direction::Press).map_err(|e| e.to_string())?;
    enigo.key(Key::Unicode('v'), Direction::Click).map_err(|e| e.to_string())?;
    enigo.key(Key::Control, Direction::Release).map_err(|e| e.to_string())?;

    Ok(())
}
