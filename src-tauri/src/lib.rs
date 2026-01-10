use tauri::Manager;
use tauri_nspanel::ManagerExt;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

use crate::window::WebviewWindowExt;

pub const KEYCAST_LABEL: &str = "main";

mod command;
mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command::show, command::hide])
        .plugin(tauri_nspanel::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(Shortcut::new(
                    Some(Modifiers::SUPER | Modifiers::SHIFT),
                    Code::Semicolon,
                ))
                .unwrap()
                .with_handler(|app, shortcut, event| {
                    if event.state == ShortcutState::Pressed
                        && shortcut.matches(Modifiers::SUPER | Modifiers::SHIFT, Code::Semicolon)
                    {
                        let window = app.get_webview_window(KEYCAST_LABEL).unwrap();

                        match app
                            .get_webview_panel(KEYCAST_LABEL)
                            .or_else(|_| window.to_spotlight_panel())
                        {
                            Ok(panel) => {
                                if panel.is_visible() {
                                    panel.hide();
                                } else {
                                    window.center_at_cursor_monitor().unwrap();
                                    panel.show_and_make_key();
                                }
                            }
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            /*
             * 앱을 액세서리 모드로 실행
             * 액세서리 모드: 앱이 dock에 표시되지 않음. 또한 메뉴바에도 앱이 노출되지 않음.
             */
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Prohibited);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
