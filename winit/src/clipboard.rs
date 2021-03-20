/// A buffer for short-term storage and transfer within and between
/// applications.
#[allow(missing_debug_implementations)]
pub struct Clipboard {
    clipboard: ClipboardState,
    ime: IMEState,
}

enum ClipboardState {
    Connected(window_clipboard::Clipboard),
    Unavailable,
}

enum IMEState {
    Connected(window_ime::IME),
    Unavailable,
}

impl Clipboard {
    /// Creates a new [`Clipboard`] for the given window.
    pub fn connect(window: &winit::window::Window) -> Clipboard {
        let clipboard = window_clipboard::Clipboard::connect(window)
            .ok()
            .map(ClipboardState::Connected)
            .unwrap_or(ClipboardState::Unavailable);

        let ime = window_ime::IME::connect(window)
            .ok()
            .map(IMEState::Connected)
            .unwrap_or(IMEState::Unavailable);

        Clipboard { clipboard, ime }
    }

    /// Reads the current content of the [`Clipboard`] as text.
    pub fn read(&self) -> Option<String> {
        match &self.clipboard {
            ClipboardState::Connected(clipboard) => clipboard.read().ok(),
            ClipboardState::Unavailable => None,
        }
    }

    /// Writes the given text contents to the [`Clipboard`].
    pub fn write(&mut self, contents: String) {
        match &mut self.clipboard {
            ClipboardState::Connected(clipboard) => {
                match clipboard.write(contents) {
                    Ok(()) => {}
                    Err(error) => {
                        log::warn!("error writing to clipboard: {}", error)
                    }
                }
            }
            ClipboardState::Unavailable => {}
        }
    }

    /// Set IME position
    pub fn set_ime_position(&self, position: iced_core::Point) {
        match &self.ime {
            IMEState::Connected(ime) => {
                ime.set_position(position.x, position.y)
            }
            IMEState::Unavailable => {}
        }
    }
}

impl iced_native::Clipboard for Clipboard {
    fn read(&self) -> Option<String> {
        self.read()
    }

    fn write(&mut self, contents: String) {
        self.write(contents)
    }

    fn set_ime_position(&self, position: iced_core::Point) {
        self.set_ime_position(position);
    }
}
