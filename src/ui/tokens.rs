use crate::app::AuthenticatorApp;
use crate::totp;

pub fn show(ui: &mut egui::Ui, app: &mut AuthenticatorApp) {
    ui.vertical(|ui| {
        super::style::large_title(ui, "Tokens", &app.theme);

        ui.add_space(8.0);

        let time = totp::current_timestamp();

        if app.tokens.is_empty() {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("No tokens yet")
                        .size(18.0)
                        .color(app.theme.text_secondary()),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("Tap + to add your first token")
                        .size(14.0)
                        .color(app.theme.text_secondary()),
                );
            });
        }

        let mut to_delete: Option<usize> = None;
        let mut copy_index: Option<usize> = None;

        for (i, token) in app.tokens.iter().enumerate() {
            let code = token.generate_code(time);
            let remaining = token.time_remaining(time);
            let progress = token.progress(time);

            egui::Frame::none()
                .fill(app.theme.bg_tertiary())
                .rounding(egui::Rounding::same(14.0))
                .inner_margin(egui::Margin::same(14.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if !token.issuer.is_empty() {
                                ui.label(
                                    egui::RichText::new(&token.issuer)
                                        .size(12.0)
                                        .color(app.theme.text_secondary()),
                                );
                            }
                            ui.label(
                                egui::RichText::new(&token.name)
                                    .size(14.0)
                                    .color(app.theme.text_primary()),
                            );
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let is_copied = app.copied_index == Some(i);
                            if ui.button(
                                egui::RichText::new("X").size(12.0).color(app.theme.destructive()),
                            ).clicked() {
                                to_delete = Some(i);
                            }

                            ui.add_space(8.0);

                            if !matches!(token.token_type, totp::TokenType::Hotp { .. }) {
                                let size = 24.0;
                                let center = egui::pos2(
                                    ui.cursor().min.x + size / 2.0,
                                    ui.cursor().min.y + size / 2.0,
                                );
                                let radius = size / 2.0;
                                ui.painter().circle_stroke(
                                    center,
                                    radius,
                                    egui::Stroke::new(2.0, app.theme.text_secondary()),
                                );
                                let filled_angle = progress * std::f32::consts::TAU;
                                let path = egui::epaint::PathShape {
                                    points: (0..=32)
                                        .map(|step| {
                                            let angle = -std::f32::consts::FRAC_PI_2
                                                + filled_angle * step as f32 / 32.0;
                                            egui::pos2(
                                                center.x + radius * angle.cos(),
                                                center.y + radius * angle.sin(),
                                            )
                                        })
                                        .collect(),
                                    closed: false,
                                    fill: egui::Color32::TRANSPARENT,
                                    stroke: egui::Stroke::new(2.5, app.theme.accent_color()),
                                };
                                ui.painter().add(path);

                                ui.add_space(size + 4.0);
                            }

                            let code_text = if is_copied {
                                "Copied!".to_string()
                            } else if matches!(token.token_type, totp::TokenType::Steam) {
                                code.clone()
                            } else {
                                let mid = code.len() / 2;
                                format!("{} {}", &code[..mid], &code[mid..])
                            };

                            let code_resp = ui.label(
                                egui::RichText::new(&code_text)
                                    .size(24.0)
                                    .color(if is_copied {
                                        app.theme.success()
                                    } else {
                                        app.theme.text_primary()
                                    })
                                    .monospace(),
                            );

                            if code_resp.clicked() {
                                copy_index = Some(i);
                            }
                        });
                    });

                    if !matches!(token.token_type, totp::TokenType::Hotp { .. }) {
                        ui.add_space(2.0);
                        ui.label(
                            egui::RichText::new(format!("{}s remaining", remaining))
                                .size(11.0)
                                .color(app.theme.text_secondary()),
                        );
                    }
                });

            ui.add_space(6.0);
        }

        if let Some(idx) = to_delete {
            app.delete_token(idx);
        } else if let Some(idx) = copy_index {
            if idx < app.tokens.len() {
                let code = app.tokens[idx].generate_code(time);
                ui.ctx().copy_text(code);
                app.copied_index = Some(idx);
            }
        }

        ui.add_space(70.0);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Bottom), |ui| {
            let fab = egui::Button::new(
                egui::RichText::new("+").size(28.0).color(egui::Color32::WHITE),
            )
            .fill(app.theme.accent_color())
            .rounding(egui::Rounding::same(28.0))
            .min_size(egui::vec2(56.0, 56.0));

            if ui.add(fab).clicked() {
                app.show_add_token = true;
            }
        });
    });
}
