use super::ime_policy::ImeMode;

#[derive(Clone, Copy)]
pub struct PlatformOptions {
    /// Disable native menus when the application renders and dispatches its own menus.
    pub native_menus: bool,
    /// Retain picker, open-URI and accepted drop grants across application restarts.
    ///
    /// Disabled by default. Requires `ohos.permission.FILE_ACCESS_PERSIST`.
    /// Restoration starts in the background after the first presented GPUI frame.
    pub persist_file_access: bool,
    /// Who receives printable hardware keys.
    ///
    /// Defaults to [`ImeMode::Attached`] (form-like behaviour: the IME owns
    /// printable keys so composition works). Applications with a terminal or a
    /// canvas want [`ImeMode::Detached`], where the same keys arrive as
    /// keystrokes and no soft keyboard appears. Can be changed at runtime with
    /// [`crate::set_ime_mode`].
    pub ime_mode: ImeMode,
}
impl Default for PlatformOptions {
    fn default() -> Self {
        Self {
            native_menus: true,
            persist_file_access: false,
            ime_mode: ImeMode::Attached,
        }
    }
}
