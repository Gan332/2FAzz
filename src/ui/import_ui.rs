use crate::app::AuthenticatorApp;

pub fn dialog(ctx: &egui::Context, app: &mut AuthenticatorApp) {
    let theme = app.theme.clone();
    egui::Window::new("Import Tokens")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .fixed_size(egui::vec2(380.0f32, 400.0f32))
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .frame(egui::Frame::none().fill(theme.bg_secondary()).rounding(egui::Rounding::same(20.0f32)))
        .show(ctx, |ui| {
            ui.add_space(8.0f32);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Import Tokens").size(18.0f32).color(theme.text_primary()));
            });
            ui.add_space(8.0f32);

            ui.label(egui::RichText::new("Paste OTP URI, Aegis JSON, or andOTP JSON:").size(13.0f32).color(theme.text_secondary()));

            ui.add_space(4.0f32);

            egui::Frame::none()
                .fill(theme.bg_tertiary())
                .rounding(egui::Rounding::same(12.0f32))
                .inner_margin(8.0f32)
                .show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width(), 180.0f32],
                        egui::TextEdit::multiline(&mut app.import_text)
                            .hint_text("otpauth://totp/...\nor paste JSON here"),
                    );
                });

            if let Some(err) = &app.import_error {
                ui.add_space(4.0f32);
                ui.label(egui::RichText::new(err).size(13.0f32).color(theme.destructive()));
            }

            ui.add_space(12.0f32);

            let can_import = !app.import_text.is_empty();
            ui.add_enabled_ui(can_import, |ui| {
                if super::style::ios_button(ui, "Import", theme.accent_color()).clicked() {
                    app.import_tokens();
                }
            });

            ui.add_space(4.0f32);

            if ui.button(
                egui::RichText::new("Cancel").size(16.0f32).color(theme.text_secondary()),
            ).clicked() {
                app.show_import = false;
                app.import_error = None;
            }
        });
}
