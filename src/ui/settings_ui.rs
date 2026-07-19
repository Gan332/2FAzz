use crate::app::AuthenticatorApp;
use crate::theme::ThemeMode;

pub fn show(ui: &mut egui::Ui, app: &mut AuthenticatorApp) {
    ui.vertical(|ui| {
        super::style::large_title(ui, "Settings", &app.theme);
        ui.add_space(8.0);

        super::style::section_header(ui, "APPEARANCE", &app.theme);

        egui::Frame::none()
            .fill(app.theme.bg_tertiary())
            .rounding(egui::Rounding::same(14.0))
            .inner_margin(14.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Theme").size(15.0).color(app.theme.text_primary()),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let is_dark = app.theme.mode == ThemeMode::Dark;
                        let label = if is_dark { "Dark" } else { "Light" };
                        let btn = egui::Button::new(
                            egui::RichText::new(label).size(14.0).color(egui::Color32::WHITE),
                        )
                        .fill(app.theme.accent_color())
                        .rounding(egui::Rounding::same(8.0));
                        if ui.add(btn).clicked() {
                            app.theme.mode = if is_dark {
                                ThemeMode::Light
                            } else {
                                ThemeMode::Dark
                            };
                            app.save_vault();
                        }
                    });
                });
            });

        ui.add_space(8.0);

        super::style::section_header(ui, "ACCENT COLOR", &app.theme);

        egui::Frame::none()
            .fill(app.theme.bg_tertiary())
            .rounding(egui::Rounding::same(14.0))
            .inner_margin(14.0)
            .show(ui, |ui| {
                let presets: [([u8; 3], &str); 8] = [
                    ([0x0a, 0x84, 0xff], "Blue"),
                    ([0x30, 0xd1, 0x58], "Green"),
                    ([0xff, 0x95, 0x00], "Orange"),
                    ([0xff, 0x37, 0x58], "Red"),
                    ([0xbf, 0x5a, 0xf2], "Purple"),
                    ([0x64, 0xd2, 0xff], "Cyan"),
                    ([0xff, 0xd6, 0x00], "Yellow"),
                    ([0xff, 0x6e, 0x8b], "Pink"),
                ];

                ui.horizontal_wrapped(|ui| {
                    for (color, name) in presets {
                        let is_selected = app.theme.accent == color;
                        let btn = egui::Button::new(
                            egui::RichText::new(name).size(12.0).color(if is_selected {
                                egui::Color32::WHITE
                            } else {
                                app.theme.text_primary()
                            }),
                        )
                        .fill(egui::Color32::from_rgb(color[0], color[1], color[2]))
                        .rounding(egui::Rounding::same(8.0))
                        .stroke(if is_selected {
                            egui::Stroke::new(2.0, egui::Color32::WHITE)
                        } else {
                            egui::Stroke::NONE
                        });

                        if ui.add(btn).clicked() {
                            app.theme.accent = color;
                            app.save_vault();
                        }
                    }
                });
            });

        ui.add_space(16.0);

        super::style::section_header(ui, "DATA", &app.theme);

        egui::Frame::none()
            .fill(app.theme.bg_tertiary())
            .rounding(egui::Rounding::same(14.0))
            .inner_margin(14.0)
            .show(ui, |ui| {
                if ui.button(
                    egui::RichText::new("Import Tokens").size(15.0).color(app.theme.accent_color()),
                ).clicked() {
                    app.show_import = true;
                }

                ui.add_space(4.0);
                let sep_rect = ui.available_rect_before_wrap();
                ui.painter().line_segment(
                    [
                        egui::pos2(sep_rect.min.x, sep_rect.min.y + 2.0),
                        egui::pos2(sep_rect.max.x, sep_rect.min.y + 2.0),
                    ],
                    egui::Stroke::new(1.0, app.theme.separator()),
                );
                ui.add_space(4.0);

                if super::style::ios_destructive_button(ui, "Delete All Data", &app.theme).clicked()
                {
                    let _ = std::fs::remove_file(crate::storage::vault_path());
                    app.tokens.clear();
                    app.passkeys.clear();
                    app.is_unlocked = false;
                    app.is_first_run = true;
                    app.vault_password.clear();
                }
            });

        ui.add_space(24.0);

        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("Authenticator v0.1.0")
                    .size(12.0)
                    .color(app.theme.text_secondary()),
            );
            ui.label(
                egui::RichText::new("Built with Rust + egui")
                    .size(11.0)
                    .color(app.theme.text_secondary()),
            );
        });
    });
}
