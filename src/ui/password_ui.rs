use crate::app::AuthenticatorApp;
use crate::password;

pub fn show(ui: &mut egui::Ui, app: &mut AuthenticatorApp) {
    ui.vertical(|ui| {
        super::style::large_title(ui, "Generator", &app.theme);

        ui.add_space(16.0f32);

        egui::Frame::none()
            .fill(app.theme.bg_tertiary())
            .rounding(egui::Rounding::same(14.0f32))
            .inner_margin(16.0f32)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    let display = if app.generated_password.is_empty() {
                        "Tap Generate".to_string()
                    } else {
                        app.generated_password.clone()
                    };
                    ui.label(
                        egui::RichText::new(&display)
                        .size(22.0f32)
                        .color(app.theme.text_primary())
                        .monospace(),
                    );
                });
            });

        ui.add_space(16.0f32);

        egui::Frame::none()
            .fill(app.theme.bg_tertiary())
            .rounding(egui::Rounding::same(14.0f32))
            .inner_margin(16.0f32)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Length").size(13.0f32).color(app.theme.text_secondary()));
                ui.add(egui::Slider::new(&mut app.password_length, 8..=64).text("characters"));
            });

        ui.add_space(8.0f32);

        egui::Frame::none()
            .fill(app.theme.bg_tertiary())
            .rounding(egui::Rounding::same(14.0f32))
            .inner_margin(16.0f32)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Character Sets").size(13.0f32).color(app.theme.text_secondary()));
                ui.add_space(4.0f32);
                ui.checkbox(&mut app.password_uppercase, "Uppercase (A-Z)");
                ui.checkbox(&mut app.password_lowercase, "Lowercase (a-z)");
                ui.checkbox(&mut app.password_digits, "Digits (0-9)");
                ui.checkbox(&mut app.password_symbols, "Symbols (!@#$%...)");
            });

        ui.add_space(16.0f32);

        if super::style::ios_button(ui, "Generate Password", app.theme.accent_color()).clicked() {
            let opts = password::PasswordOptions {
                length: app.password_length,
                uppercase: app.password_uppercase,
                lowercase: app.password_lowercase,
                digits: app.password_digits,
                symbols: app.password_symbols,
            };
            app.generated_password = password::generate_password(&opts);
        }

        if !app.generated_password.is_empty() {
            ui.add_space(8.0f32);
            if super::style::ios_button(ui, "Copy Password", app.theme.accent_color()).clicked() {
                ui.ctx().copy_text(app.generated_password.clone());
            }
        }
    });
}
