use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTheme {
    pub mode: ThemeMode,
    pub accent: [u8; 3],
}

impl Default for AppTheme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Dark,
            accent: [0x0a, 0x84, 0xff],
        }
    }
}

impl AppTheme {
    pub fn accent_color(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.accent[0], self.accent[1], self.accent[2])
    }

    pub fn bg_primary(&self) -> egui::Color32 {
        match self.mode {
            ThemeMode::Light => egui::Color32::from_rgb(0xf2, 0xf2, 0xf7),
            ThemeMode::Dark => egui::Color32::from_rgb(0x00, 0x00, 0x00),
        }
    }

    pub fn bg_secondary(&self) -> egui::Color32 {
        match self.mode {
            ThemeMode::Light => egui::Color32::WHITE,
            ThemeMode::Dark => egui::Color32::from_rgb(0x1c, 0x1c, 0x1e),
        }
    }

    pub fn bg_tertiary(&self) -> egui::Color32 {
        match self.mode {
            ThemeMode::Light => egui::Color32::from_rgb(0xff, 0xff, 0xff),
            ThemeMode::Dark => egui::Color32::from_rgb(0x2c, 0x2c, 0x2e),
        }
    }

    pub fn text_primary(&self) -> egui::Color32 {
        match self.mode {
            ThemeMode::Light => egui::Color32::BLACK,
            ThemeMode::Dark => egui::Color32::WHITE,
        }
    }

    pub fn text_secondary(&self) -> egui::Color32 {
        match self.mode {
            ThemeMode::Light => egui::Color32::from_rgb(0x8e, 0x8e, 0x93),
            ThemeMode::Dark => egui::Color32::from_rgb(0x8e, 0x8e, 0x93),
        }
    }

    pub fn separator(&self) -> egui::Color32 {
        match self.mode {
            ThemeMode::Light => egui::Color32::from_rgb(0xc6, 0xc6, 0xc8),
            ThemeMode::Dark => egui::Color32::from_rgb(0x38, 0x38, 0x3a),
        }
    }

    pub fn destructive(&self) -> egui::Color32 {
        match self.mode {
            ThemeMode::Light => egui::Color32::from_rgb(0xff, 0x3b, 0x30),
            ThemeMode::Dark => egui::Color32::from_rgb(0xff, 0x45, 0x3a),
        }
    }

    pub fn success(&self) -> egui::Color32 {
        match self.mode {
            ThemeMode::Light => egui::Color32::from_rgb(0x34, 0xc7, 0x59),
            ThemeMode::Dark => egui::Color32::from_rgb(0x30, 0xd1, 0x58),
        }
    }

    pub fn apply_to_egui(&self, ctx: &egui::Context) {
        let mut visuals = match self.mode {
            ThemeMode::Light => egui::Visuals::light(),
            ThemeMode::Dark => egui::Visuals::dark(),
        };

        visuals.window_rounding = egui::Rounding::same(14.0f32);
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(12.0f32);
        visuals.widgets.inactive.rounding = egui::Rounding::same(12.0f32);
        visuals.widgets.active.rounding = egui::Rounding::same(12.0f32);
        visuals.widgets.hovered.rounding = egui::Rounding::same(12.0f32);

        visuals.widgets.noninteractive.bg_fill = self.bg_tertiary();
        visuals.widgets.inactive.bg_fill = self.bg_tertiary();
        visuals.extreme_bg_color = self.bg_primary();
        visuals.window_fill = self.bg_secondary();
        visuals.panel_fill = self.bg_primary();

        visuals.selection.bg_fill = self.accent_color();
        visuals.selection.stroke = egui::Stroke::new(1.0f32, self.accent_color());

        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0f32, 6.0f32);
        style.spacing.button_padding = egui::vec2(16.0f32, 8.0f32);
        style.interaction.show_tooltips_only_when_still = false;
        ctx.set_style(style);
    }
}
