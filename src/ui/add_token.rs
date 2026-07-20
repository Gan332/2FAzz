use crate::app::AuthenticatorApp;
use crate::totp::TokenType;

pub fn dialog(ctx: &egui::Context, app: &mut AuthenticatorApp) {
    let theme = app.theme.clone();
    egui::Window::new("Add Token")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .fixed_size(egui::vec2(360.0f32, 480.0f32))
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .frame(egui::Frame::none().fill(theme.bg_secondary()).rounding(egui::Rounding::same(20.0f32)))
        .show(ctx, |ui| {
            ui.add_space(8.0f32);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Add Token").size(18.0f32).color(theme.text_primary()));
            });
            ui.add_space(4.0f32);

            egui::Frame::none()
                .fill(theme.bg_tertiary())
                .rounding(egui::Rounding::same(12.0f32))
                .inner_margin(12.0f32)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Type").size(13.0f32).color(theme.text_secondary()));
                    ui.horizontal(|ui| {
                        let types = [
                            ("TOTP", TokenType::Totp),
                            ("HOTP", TokenType::Hotp { counter: 0 }),
                            ("Steam", TokenType::Steam),
                        ];
                        for (label, tt) in types {
                            let is_sel = matches_token_type(&app.new_token_type, &tt);
                            let btn = egui::Button::new(
                                egui::RichText::new(label).size(13.0f32).color(if is_sel {
                                    egui::Color32::WHITE
                                } else {
                                    theme.text_primary()
                                }),
                            )
                            .fill(if is_sel { theme.accent_color() } else { theme.bg_primary() })
                            .rounding(egui::Rounding::same(8.0f32));

                            if ui.add(btn).clicked() {
                                app.new_token_type = tt;
                            }
                        }
                    });
                });

            ui.add_space(8.0f32);

            egui::Frame::none()
                .fill(theme.bg_tertiary())
                .rounding(egui::Rounding::same(12.0f32))
                .inner_margin(12.0f32)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Issuer").size(13.0f32).color(theme.text_secondary()));
                    ui.add_sized(
                        [ui.available_width(), 32.0f32],
                        egui::TextEdit::singleline(&mut app.new_token_issuer).hint_text("Google, GitHub..."),
                    );
                    ui.add_space(4.0f32);
                    ui.label(egui::RichText::new("Name").size(13.0f32).color(theme.text_secondary()));
                    ui.add_sized(
                        [ui.available_width(), 32.0f32],
                        egui::TextEdit::singleline(&mut app.new_token_name).hint_text("user@email.com"),
                    );
                    ui.add_space(4.0f32);
                    ui.label(egui::RichText::new("Secret").size(13.0f32).color(theme.text_secondary()));
                    ui.add_sized(
                        [ui.available_width(), 32.0f32],
                        egui::TextEdit::singleline(&mut app.new_token_secret)
                            .hint_text("Base32 secret"),
                    );
                });

            ui.add_space(8.0f32);

            egui::Frame::none()
                .fill(theme.bg_tertiary())
                .rounding(egui::Rounding::same(12.0f32))
                .inner_margin(12.0f32)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Digits").size(13.0f32).color(theme.text_secondary()));
                        ui.add(egui::DragValue::new(&mut app.new_token_digits).range(4.0_f32..=10.0_f32));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new("Period").size(13.0f32).color(theme.text_secondary()));
                            ui.add(egui::DragValue::new(&mut app.new_token_period).range(5.0_f32..=120.0_f32));
                            ui.label("s");
                        });
                    });
                });

            ui.add_space(16.0f32);

            let can_add = !app.new_token_secret.is_empty() && !app.new_token_name.is_empty();
            ui.add_enabled_ui(can_add, |ui| {
                if super::style::ios_button(ui, "Add Token", theme.accent_color()).clicked() {
                    app.add_token();
                }
            });

            ui.add_space(4.0f32);

            if ui.button(
                egui::RichText::new("Cancel").size(16.0f32).color(theme.text_secondary()),
            ).clicked() {
                app.show_add_token = false;
            }
        });
}

pub fn passkey_dialog(ctx: &egui::Context, app: &mut AuthenticatorApp) {
    let theme = app.theme.clone();
    egui::Window::new("Add Passkey")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .fixed_size(egui::vec2(360.0f32, 360.0f32))
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .frame(egui::Frame::none().fill(theme.bg_secondary()).rounding(egui::Rounding::same(20.0f32)))
        .show(ctx, |ui| {
            ui.add_space(8.0f32);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Add Passkey").size(18.0f32).color(theme.text_primary()));
            });
            ui.add_space(12.0f32);

            egui::Frame::none()
                .fill(theme.bg_tertiary())
                .rounding(egui::Rounding::same(12.0f32))
                .inner_margin(12.0f32)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Relying Party ID").size(13.0f32).color(theme.text_secondary()));
                    ui.add_sized(
                        [ui.available_width(), 32.0f32],
                        egui::TextEdit::singleline(&mut app.new_passkey_rp).hint_text("example.com"),
                    );
                    ui.add_space(4.0f32);
                    ui.label(egui::RichText::new("Username").size(13.0f32).color(theme.text_secondary()));
                    ui.add_sized(
                        [ui.available_width(), 32.0f32],
                        egui::TextEdit::singleline(&mut app.new_passkey_username).hint_text("user@email.com"),
                    );
                    ui.add_space(4.0f32);
                    ui.label(egui::RichText::new("Display Name").size(13.0f32).color(theme.text_secondary()));
                    ui.add_sized(
                        [ui.available_width(), 32.0f32],
                        egui::TextEdit::singleline(&mut app.new_passkey_display).hint_text("John Doe"),
                    );
                });

            ui.add_space(16.0f32);

            let can_add = !app.new_passkey_rp.is_empty() && !app.new_passkey_username.is_empty();
            ui.add_enabled_ui(can_add, |ui| {
                if super::style::ios_button(ui, "Add Passkey", theme.accent_color()).clicked() {
                    app.add_passkey();
                }
            });

            ui.add_space(4.0f32);

            if ui.button(
                egui::RichText::new("Cancel").size(16.0f32).color(theme.text_secondary()),
            ).clicked() {
                app.show_add_passkey = false;
            }
        });
}

fn matches_token_type(a: &TokenType, b: &TokenType) -> bool {
    match (a, b) {
        (TokenType::Totp, TokenType::Totp) => true,
        (TokenType::Hotp { .. }, TokenType::Hotp { .. }) => true,
        (TokenType::Steam, TokenType::Steam) => true,
        _ => false,
    }
}
