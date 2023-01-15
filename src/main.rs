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
