// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::{ CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowEvent };

fn main() {
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(exit);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu).with_tooltip("Weather Status"))
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }

            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "settings" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }

                "exit" => {
                    std::process::exit(0);
                }

                _ => {}
            }

            _ => {}
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                // Don't kill the app when closing the window
                event.window().hide().unwrap();
                api.prevent_close();
            }

//            WindowEvent::Focused(false) => {
//                // Automatically hide the window when clicking outside
//                event.window().hide().unwrap();
//            }

            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}