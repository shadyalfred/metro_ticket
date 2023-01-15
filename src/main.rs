#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{self, NativeOptions};

use metro_ticket::app::MetroApp;

fn main() {
    let app = MetroApp::new();

    let win_options = NativeOptions::default();

    eframe::run_native(
        "Metro Ticket App",
        win_options,
        Box::new(|_cc| Box::new(app)),
    );
}
