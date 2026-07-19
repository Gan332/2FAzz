use crate::app::AuthenticatorApp;

pub fn show(ui: &mut egui::Ui, app: &mut AuthenticatorApp) {
    ui.vertical(|ui| {
        super::style::large_title(ui, "Passkeys", &app.theme);

        ui.add_space(8.0);

        if app.passkeys.is_empty() {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("No passkeys stored")
                        .size(18.0)
                        .color(app.theme.text_secondary()),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("Store your passkey credentials locally")
                        .size(14.0)
                        .color(app.theme.text_secondary()),
                );
            });
        }

        let mut to_delete: Option<usize> = None;

        for (i, pk) in app.passkeys.iter().enumerate() {
            egui::Frame::none()
                .fill(app.theme.bg_tertiary())
                .rounding(egui::Rounding::same(14.0))
                .inner_margin(14.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(&pk.rp_id)
                                    .size(14.0)
                                    .color(app.theme.text_primary()),
                            );
                            ui.label(
                                egui::RichText::new(&pk.username)
                                    .size(12.0)
                                    .color(app.theme.text_secondary()),
                            );
                            if pk.display_name != pk.username {
                                ui.label(
                                    egui::RichText::new(&pk.display_name)
                                        .size(12.0)
                                        .color(app.theme.text_secondary()),
                                );
                            }
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(
                                egui::RichText::new("\u{2715}").size(14.0).color(app.theme.destructive()),
                            ).clicked() {
                                to_delete = Some(i);
                            }

                            let cred_short = if pk.credential_id.len() > 12 {
                                format!("{}...", &pk.credential_id[..12])
                            } else {
                                pk.credential_id.clone()
                            };
                            ui.label(
                                egui::RichText::new(cred_short)
                                    .size(11.0)
                                    .color(app.theme.text_secondary())
                                    .monospace(),
                            );
                        });
                    });
                });

            ui.add_space(6.0);
        }

        if let Some(idx) = to_delete {
            app.delete_passkey(idx);
        }

        ui.add_space(70.0);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
            let fab = egui::Button::new(
                egui::RichText::new("+").size(28.0).color(egui::Color32::WHITE),
            )
            .fill(app.theme.accent_color())
            .rounding(egui::Rounding::same(28.0))
            .min_size(egui::vec2(56.0, 56.0));

            if ui.add(fab).clicked() {
                app.show_add_passkey = true;
            }
        });
    });
}
