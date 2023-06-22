// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, Manager, Monitor, PhysicalPosition, PhysicalSize, SystemTray, SystemTrayEvent,
    Window,
};

fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    let window: Window = app.get_window("main").unwrap();

    if let SystemTrayEvent::LeftClick { position, .. } = event {
        let outer_size: PhysicalSize<u32> = window.outer_size().unwrap();

        if window.is_visible().unwrap() {
            window.hide().unwrap();
            window.emit("window:hidden", false).unwrap();
        } else {
            window.show().unwrap();
            window.set_focus().unwrap();
        }

        window
            .set_position(PhysicalPosition {
                x: position.x,
                y: position.y,
            })
            .unwrap();

        let current_monitor: Monitor = window.current_monitor().unwrap().unwrap();
        let screen_size: &PhysicalSize<u32> = current_monitor.size();
        let screen_position: &PhysicalPosition<i32> = current_monitor.position();

        let x: f64 = f64::from(screen_position.x) + f64::from(screen_size.width)
            - f64::from(outer_size.width);
        let y: f64 = f64::from(screen_position.y) + f64::from(screen_size.height)
            - f64::from(outer_size.height)
            - 32.0;

        window.set_position(PhysicalPosition { x, y }).unwrap();
    }
}

fn main() {
    let system_tray: SystemTray = SystemTray::new();

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(handle_system_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
