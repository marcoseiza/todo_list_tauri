use cocoa::base::BOOL;
use objc::{msg_send, sel, sel_impl};
use tauri::Window;

enum NSWindowTitleVisibility {
    NSWindowTitleVisible = 0,
    NSWindowTitleHidden = 1,
}

impl From<bool> for NSWindowTitleVisibility {
    fn from(val: bool) -> Self {
        if val {
            NSWindowTitleVisibility::NSWindowTitleVisible
        } else {
            NSWindowTitleVisibility::NSWindowTitleHidden
        }
    }
}

/// Applies transparency and title visibility to window.
///
/// # Examples
///
/// ```rust,no_run
/// #[cfg(target_os = "macos")]
/// use::macos_titlebar::apply_title_bar_options
///
/// fn main() {
///   tauri::Builder::default()
///     .setup(|app| {
///         let window = app.get_window("main").unwrap();
///         #[cfg(target_os = "macos")]
///         apply_title_bar_options(&window, true, false).expect(
///             "Unsupported platform! 'apply_transparent_title_bar' is only supported on macOS",
///         );
///         Ok(())
///     })
///     .expect("error while running tauri application");
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn apply_title_bar_options(
    window: &Window,
    transparent: bool,
    visible: bool,
) -> Result<(), tauri::Error> {
    window.with_webview(move |webview| {
        #[cfg(target_os = "macos")]
        unsafe {
            let ns_window: cocoa::base::id = webview.ns_window();
            let transparency: BOOL = transparent.into();
            let visibility: NSWindowTitleVisibility = visible.into();
            let () = msg_send![ns_window, setTitlebarAppearsTransparent: transparency];
            let () = msg_send![ns_window, setTitleVisibility: visibility];
        }
    })
}
