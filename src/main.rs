#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::Vec2;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(Vec2::new(1920.00, 1080.00)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "boardx",
        native_options,
        Box::new(|cc| Box::new(boardx::App::new(cc))),
    );
}
