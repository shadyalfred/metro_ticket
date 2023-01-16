use eframe::App;
use egui::{Align, CentralPanel, Color32, Layout, ScrollArea, SidePanel, TextEdit, TopBottomPanel};

use crate::{metro::Metro, ticket::Ticket};

pub struct MetroApp {
    metro: Metro,
    start_stations_filter: String,
    selected_start_station: String,
    destination_stations_filter: String,
    selected_destination_station: String,
    all_stations: Vec<String>,
}

impl App for MetroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut filtered_start_stations: Vec<&str> = Vec::new();
        let mut filtered_destination_stations: Vec<&str> = Vec::new();

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Metro Ticket App");
            });
        });

        // Start
        SidePanel::left("Start").show(ctx, |ui| {
            ui.add(
                TextEdit::singleline(&mut self.start_stations_filter)
                    .hint_text("Filter start stations"),
            );

            ui.label("Start");

            if !self.start_stations_filter.is_empty() {
                filtered_start_stations = self
                    .all_stations
                    .iter()
                    .filter(|station| {
                        station
                            .to_lowercase()
                            .contains(&self.start_stations_filter.to_lowercase())
                    })
                    .map(|s| s.as_ref())
                    .collect();
            } else {
                if filtered_start_stations.len() != self.all_stations.len() {
                    filtered_start_stations =
                        self.all_stations.iter().map(|s| s.as_ref()).collect();
                }
            }

            ScrollArea::vertical().show(ui, |ui| {
                for station in filtered_start_stations {
                    ui.selectable_value(
                        &mut self.selected_start_station,
                        station.to_string(),
                        station,
                    );
                }
            });
        });

        // Destination
        SidePanel::right("Destination").show(ctx, |ui| {
            ui.vertical(|ui| {
                    ui.add(
                        TextEdit::singleline(&mut self.destination_stations_filter)
                            .hint_text("Filter destination stations"),
                    );

                    ui.label("Destination");

                if !self.destination_stations_filter.is_empty() {
                    filtered_destination_stations = self
                        .all_stations
                        .iter()
                        .filter(|station| {
                            station
                                .to_lowercase()
                                .contains(&self.destination_stations_filter.to_lowercase())
                        })
                        .map(|s| s.as_ref())
                        .collect();
                } else {
                    if filtered_destination_stations.len() != self.all_stations.len() {
                        filtered_destination_stations =
                            self.all_stations.iter().map(|s| s.as_ref()).collect();
                    }
                }

                        ScrollArea::vertical().show(ui, |ui| {
                            for station in filtered_destination_stations {
                                ui.selectable_value(
                                    &mut self.selected_destination_station,
                                    station.to_string(),
                                    station,
                                );
                            }
                        });
            });
        });

        // Ticket
        CentralPanel::default().show(ctx, |ui| {
            if !self.selected_start_station.is_empty()
                && !self.selected_destination_station.is_empty()
            {
                let (ticket, path, lines) = self.metro.calculate_ticket(
                    &self.selected_start_station,
                    &self.selected_destination_station,
                );

                match ticket {
                    Ticket::Yellow => ui.visuals_mut().override_text_color = Some(Color32::YELLOW),
                    Ticket::Green => ui.visuals_mut().override_text_color = Some(Color32::GREEN),
                    Ticket::Purple => {
                        ui.visuals_mut().override_text_color = Some(Color32::LIGHT_RED)
                    }
                }

                ui.centered_and_justified(|ui| {
                    ui.heading(format!(
                        "{:?} Ticket\n{} Stations\n{} Lines",
                        ticket,
                        path.len() - 1,
                        lines
                    ));
                });

                ui.reset_style();
            }
        });
    }
}

impl MetroApp {
    pub fn new() -> Self {
        let metro = Metro::new();
        let all_stations = metro.get_all_stations();

        return Self {
            metro,
            all_stations,
            start_stations_filter: "".to_string(),
            selected_start_station: "".to_string(),
            destination_stations_filter: "".to_string(),
            selected_destination_station: "".to_string(),
        };
    }
}
