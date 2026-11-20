//! Whether this process lets the HarmonyOS input method own the hardware
//! keyboard.
//!
//! HarmonyOS binds one IME editor to the focused window and, once a text client
//! is attached, the IME takes every printable key: the application is only given
//! the committed text. Measured on the 2in1 device, `set_consume_key_events`
//! does not change that — with it set to `false` an ASCII `d` still arrived as
//! `insert_text "d"` and never as `KeyDown`, because the editor (not the flag)
//! owns the key. That is right for a form and wrong for a terminal, where `d`
//! must reach the application as a keystroke. The two answers are therefore
//! explicit rather than constant:
//!
//! * [`ImeMode::Detached`] — no IME editor is bound: keys arrive as
//!   `KeyDown`/`KeyUp` with their unicode, the soft keyboard hides, and
//!   composition is impossible.
//! * [`ImeMode::Attached`] — the IME is bound: CJK composition works and
//!   printable keys belong to the IME.
//!
//! The mode is process-wide because the platform routes keys to the focused
//! window's single IME editor; an application cannot have two answers at once.
//! It is applied on the next frame, not on the next attachment, so switching
//! releases (or re-requests) the keyboard immediately.

use std::sync::atomic::{AtomicU8, Ordering};

/// Who receives a printable hardware key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImeMode {
    /// No IME editor bound: GPUI sees the keystroke, the IME sees nothing.
    Detached,
    /// IME editor bound: the IME composes, the application sees the result.
    Attached,
}

const DETACHED: u8 = 0;
const ATTACHED: u8 = 1;

static MODE: AtomicU8 = AtomicU8::new(ATTACHED);

/// Set the process-wide IME mode. Takes effect on the next frame.
pub fn set_ime_mode(mode: ImeMode) {
    let raw = match mode {
        ImeMode::Detached => DETACHED,
        ImeMode::Attached => ATTACHED,
    };
    MODE.store(raw, Ordering::Release);
}

/// The current process-wide IME mode.
pub fn ime_mode() -> ImeMode {
    if MODE.load(Ordering::Acquire) == ATTACHED {
        ImeMode::Attached
    } else {
        ImeMode::Detached
    }
}
