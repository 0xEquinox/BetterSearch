use eframe::egui::{self, WidgetText};

pub fn app_entry_ui(
    ui: &mut eframe::egui::Ui,
    on: &mut bool,
    name: String,
    index: i32,
) -> egui::Response {
    let mut size = egui::Vec2::new(580.0, 45.0);

    let (rect, mut response) = ui.allocate_exact_size(size, egui::Sense::click());

    if response.clicked() {
        *on = true;
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, *on);
        ui.painter()
            .rect(rect, 0.0, visuals.bg_fill, visuals.bg_stroke);
        ui.painter().text(
            egui::Pos2::new(12.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            name,
            egui::FontId::default(),
            egui::Color32::WHITE,
        );
    }

    response
}

pub fn app_entry(on: &mut bool, name: String, index: i32) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| app_entry_ui(ui, on, name, index)
}
