use std::io;
use std::{fs::File, path::PathBuf};

use crate::componenets;
use crate::executor;
use eframe::{egui, run_native, NativeOptions};
use egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Shortcut {
    pub(crate) name: String,
    pub(crate) path: String,
}

#[derive(Default)]
struct Interface {
    input: String,
    displayed_shortcuts: Vec<Shortcut>,
    shortcut_path: PathBuf,
}

impl eframe::App for Interface {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            frame.set_decorations(false);
            ctx.set_visuals(egui::style::Visuals::dark());

            let response = ui.add_sized(
                egui::Vec2::new(ui.available_width(), 75 as f32),
                egui::TextEdit::singleline(&mut self.input)
                    .frame(false)
                    .font(egui::FontId::new(55 as f32, egui::FontFamily::Proportional))
                    .hint_text(egui::WidgetText::from("Search...")),
            );

            if response.changed() && self.input != "".to_owned() {
                self.displayed_shortcuts =
                    executor::search(&self.input, &mut reload_shortcuts(&self.shortcut_path));
            }
            if self.input.len() > 0 {
                for i in 0..self.displayed_shortcuts.len() {
                    frame.set_window_size(egui::Vec2::new(600 as f32, (75 + 50 * (i + 1)) as f32));
                    ui.add(componenets::app_entry(
                        &mut false,
                        self.displayed_shortcuts[i].name.clone(),
                        i as i32,
                    ));
                }
            } else {
                frame.set_window_size(egui::Vec2::new(600.0, 75.0));
            }
        });
    }
}

pub fn run_interface(shortcut_file: PathBuf) {
    let app = Interface {
        input: "".to_owned(),
        displayed_shortcuts: Vec::new(),
        shortcut_path: shortcut_file.clone(),
    };
    let window_option = NativeOptions {
        always_on_top: true,
        maximized: false,
        decorated: false,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_size: Some(egui::Vec2::new(600 as f32, 75 as f32)),
        initial_window_pos: Some(egui::Pos2::new(650 as f32, 100 as f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: false,
        transparent: false,
        ..Default::default()
    };
    executor::pupulate_shortcuts(shortcut_file);
    run_native("Better Search", window_option, Box::new(|cc| Box::new(app)));
}

fn reload_shortcuts(shortcut_file: &PathBuf) -> Vec<Shortcut> {
    let shortcuts_json = File::open(&shortcut_file).unwrap();

    // Read shortcuts from file
    let shortcuts: Vec<Shortcut> =
        serde_json::from_reader(&shortcuts_json).expect("Failed to parse json");

    shortcuts
}

fn save_shortcuts(shortcut_file: &str, shortcuts: &Vec<Shortcut>) {
    let shortcuts_json = File::create(&shortcut_file).unwrap();

    // Write shortcuts to file
    serde_json::to_writer_pretty(shortcuts_json, shortcuts).expect("Failed to write json");
}
