#![windows_subsystem = "windows"] // Don't go through console on Windows.

use eframe::egui::widgets::plot::{Line, Plot, Value, Values};

fn main() {
    eframe::run_native(
        "test",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(AppMain::new(cc))),
    );
}

pub struct AppMain {}

impl AppMain {
    fn new(_cc: &eframe::CreationContext) -> AppMain {
        AppMain {}
    }
}

impl eframe::App for AppMain {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::containers::panel::CentralPanel::default().show(ctx, |ui| {
            Plot::new("diagonal line")
                .data_aspect(1.0)
                .show(ui, |plot| {
                    plot.line(Line::new(Values::from_values(vec![
                        Value::new(0.0, 0.0),
                        Value::new(1.0, 1.0),
                    ])))
                });
        });
    }
}
