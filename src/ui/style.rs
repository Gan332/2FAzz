use crate::app::{AuthenticatorApp, Tab};

pub fn unlock_screen(ctx: &egui::Context, app: &mut AuthenticatorApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let available = ui.available_size();
        ui.vertical_centered(|ui| {
            ui.add_space(available.y * 0.25f32);

            ui.heading("Authenticator");
            ui.add_space(4.0f32);
            ui.label(egui::RichText::new("Secure 2FA Vault").color(app.theme.text_secondary()));
            ui.add_space(32.0f32);

            if app.is_first_run {
                ui.label("Set a password to protect your vault:");
            } else {
                ui.label("Enter your vault password:");
            }
            ui.add_space(8.0f32);

            let width = 280.0f32;
            let pwd_resp = ui.add_sized(
                [width, 36.0],
                egui::TextEdit::singleline(&mut app.vault_password)
                    .password(true)
                    .hint_text("Password"),
            );

            if pwd_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                app.try_unlock();
            }

            if let Some(err) = &app.unlock_error {
                ui.add_space(4.0f32);
                ui.label(egui::RichText::new(err).color(app.theme.destructive()));
            }

            ui.add_space(12.0f32);
            let btn_text = if app.is_first_run { "Create Vault" } else { "Unlock" };
            let btn = egui::Button::new(
                egui::RichText::new(btn_text).color(egui::Color32::WHITE),
            )
            .fill(app.theme.accent_color())
            .rounding(egui::Rounding::same(12.0f32));
            if ui.add_sized([width, 40.0], btn).clicked() {
                app.try_unlock();
            }
        });
    });
}

pub fn tab_bar(ui: &mut egui::Ui, app: &mut AuthenticatorApp) {
    let tabs = [
        (Tab::Tokens, "Tokens"),
        (Tab::Passkeys, "Passkeys"),
        (Tab::Generator, "Generator"),
        (Tab::Settings, "Settings"),
    ];

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(0.0f32, 0.0f32);

        for (tab, label) in tabs {
            let is_selected = app.current_tab == tab;
            let btn = egui::Button::new(
                egui::RichText::new(label).size(13.0f32).color(if is_selected {
                    egui::Color32::WHITE
                } else {
                    app.theme.text_secondary()
                }),
            )
            .fill(if is_selected { app.theme.accent_color() } else { app.theme.bg_primary() })
            .rounding(egui::Rounding::same(10.0f32))
            .min_size(egui::vec2(ui.available_width() / tabs.len() as f32, 40.0));

            if ui.add(btn).clicked() {
                app.current_tab = tab;
            }
        }
    });
}

pub fn ios_card(ui: &mut egui::Ui, theme: &crate::theme::AppTheme, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(theme.bg_tertiary())
        .rounding(egui::Rounding::same(14.0f32))
        .inner_margin(16.0f32)
        .show(ui, add_contents);
}

pub fn ios_button(
    ui: &mut egui::Ui,
    text: impl Into<egui::WidgetText>,
    accent: egui::Color32,
) -> egui::Response {
    ui.add_sized(
        [ui.available_width(), 44.0],
        egui::Button::new(
            egui::RichText::new(text.into()).color(egui::Color32::WHITE).size(16.0f32),
        )
        .fill(accent)
        .rounding(egui::Rounding::same(12.0f32)),
    )
}

pub fn ios_destructive_button(
    ui: &mut egui::Ui,
    text: impl Into<egui::WidgetText>,
    theme: &crate::theme::AppTheme,
) -> egui::Response {
    ui.add_sized(
        [ui.available_width(), 44.0],
        egui::Button::new(
            egui::RichText::new(text.into()).color(egui::Color32::WHITE).size(16.0f32),
        )
        .fill(theme.destructive())
        .rounding(egui::Rounding::same(12.0f32)),
    )
}

pub fn large_title(ui: &mut egui::Ui, text: &str, theme: &crate::theme::AppTheme) {
    ui.label(
        egui::RichText::new(text)
            .size(28.0f32)
            .color(theme.text_primary())
            .strong(),
    );
}

pub fn section_header(ui: &mut egui::Ui, text: &str, theme: &crate::theme::AppTheme) {
    ui.add_space(4.0f32);
    ui.label(
        egui::RichText::new(text)
            .size(13.0f32)
            .color(theme.text_secondary()),
    );
    ui.add_space(2.0f32);
}
