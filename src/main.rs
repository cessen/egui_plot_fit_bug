#![windows_subsystem = "windows"] // Don't go through console on Windows.

use eframe::egui::widgets::plot::{Line, Plot, Value, Values};

fn main() {
    eframe::run_native(Box::new(AppMain {}), eframe::NativeOptions::default());
}

pub struct AppMain {}

impl eframe::epi::App for AppMain {
    fn name(&self) -> &str {
        "test"
    }

    fn setup(
        &mut self,
        _ctx: &eframe::egui::CtxRef,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn save(&mut self, _storage: &mut dyn eframe::epi::Storage) {}

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
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
