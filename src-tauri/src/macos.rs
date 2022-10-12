use cocoa::appkit::{NSWindow, NSWindowTitleVisibility};
use cocoa::base::id;
// use objc::*;
use tauri::Window;

/// Applies transparency and title visibility to window.
#[cfg(target_os = "macos")]
pub fn apply_title_bar_options(
    window: &Window,
    transparent: bool,
    visible: bool,
) -> Result<(), tauri::Error> {
    window.with_webview(move |webview| {
        #[cfg(target_os = "macos")]
        unsafe {
            let ns_window: id = webview.ns_window();
            if visible {
                ns_window.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleVisible);
            } else {
                ns_window.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);
            }
            ns_window.setTitlebarAppearsTransparent_(transparent);
        }
    })
}
