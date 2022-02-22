mod headlines;
use eframe::{NativeOptions, run_native, epi::{App}, egui::{CentralPanel, ScrollArea, Vec2}};
use crate::headlines::*;

impl App for Headlines {
    fn setup (&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame, _storage: Option<&dyn eframe::epi::Storage>,) {
                self.configure_fonts(ctx);            
            }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        self.render_footer(ctx);
        CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ScrollArea::vertical().show(ui, |ui| {
                self.render_news_cards(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app: Headlines = Headlines::new();

    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    
    run_native(Box::new(app), win_option);
}
