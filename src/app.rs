use crate::passkey::PasskeyEntry;
use crate::storage;
use crate::theme::AppTheme;
use crate::totp::{TokenEntry, TokenType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Tokens,
    Passkeys,
    Generator,
    Settings,
}

pub struct AuthenticatorApp {
    pub tokens: Vec<TokenEntry>,
    pub passkeys: Vec<PasskeyEntry>,
    pub current_tab: Tab,
    pub theme: AppTheme,

    pub show_add_token: bool,
    pub show_add_passkey: bool,
    pub show_import: bool,

    pub new_token_name: String,
    pub new_token_secret: String,
    pub new_token_issuer: String,
    pub new_token_type: TokenType,
    pub new_token_digits: u32,
    pub new_token_period: u32,

    pub new_passkey_rp: String,
    pub new_passkey_username: String,
    pub new_passkey_display: String,

    pub import_text: String,
    pub import_error: Option<String>,

    pub password_length: u32,
    pub password_uppercase: bool,
    pub password_lowercase: bool,
    pub password_digits: bool,
    pub password_symbols: bool,
    pub generated_password: String,

    pub vault_password: String,
    pub is_unlocked: bool,
    pub unlock_error: Option<String>,
    pub is_first_run: bool,

    pub accent_picker: bool,
    pub copied_index: Option<usize>,

    pub delete_confirm: Option<usize>,
}

impl AuthenticatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = AppTheme::default();
        theme.apply_to_egui(&cc.egui_ctx);

        let is_first_run = !storage::vault_exists();

        Self {
            tokens: Vec::new(),
            passkeys: Vec::new(),
            current_tab: Tab::Tokens,
            theme,

            show_add_token: false,
            show_add_passkey: false,
            show_import: false,

            new_token_name: String::new(),
            new_token_secret: String::new(),
            new_token_issuer: String::new(),
            new_token_type: TokenType::Totp,
            new_token_digits: 6,
            new_token_period: 30,

            new_passkey_rp: String::new(),
            new_passkey_username: String::new(),
            new_passkey_display: String::new(),

            import_text: String::new(),
            import_error: None,

            password_length: 20,
            password_uppercase: true,
            password_lowercase: true,
            password_digits: true,
            password_symbols: true,
            generated_password: String::new(),

            vault_password: String::new(),
            is_unlocked: false,
            unlock_error: None,
            is_first_run,

            accent_picker: false,
            copied_index: None,
            delete_confirm: None,
        }
    }

    pub fn try_unlock(&mut self) {
        if self.vault_password.is_empty() {
            return;
        }
        if self.is_first_run {
            self.is_unlocked = true;
            self.save_vault();
            return;
        }
        match storage::load_vault(&self.vault_password) {
            Ok(data) => {
                self.tokens = data.tokens;
                self.passkeys = data.passkeys;
                self.theme = data.theme;
                self.is_unlocked = true;
                self.unlock_error = None;
            }
            Err(e) => {
                self.unlock_error = Some(e);
            }
        }
    }

    pub fn save_vault(&self) {
        let data = storage::VaultData {
            tokens: self.tokens.clone(),
            passkeys: self.passkeys.clone(),
            theme: self.theme.clone(),
        };
        if let Err(e) = storage::save_vault(&data, &self.vault_password) {
            log::error!("Failed to save vault: {}", e);
        }
    }

    pub fn add_token(&mut self) {
        let entry = TokenEntry {
            name: self.new_token_name.clone(),
            issuer: self.new_token_issuer.clone(),
            secret: self.new_token_secret.clone(),
            token_type: self.new_token_type.clone(),
            digits: self.new_token_digits,
            period: self.new_token_period as u64,
        };
        self.tokens.push(entry);
        self.new_token_name.clear();
        self.new_token_secret.clear();
        self.new_token_issuer.clear();
        self.new_token_type = TokenType::Totp;
        self.new_token_digits = 6;
        self.new_token_period = 30;
        self.show_add_token = false;
        self.save_vault();
    }

    pub fn add_passkey(&mut self) {
        let entry = PasskeyEntry::new(
            &self.new_passkey_rp,
            &self.new_passkey_username,
            &self.new_passkey_display,
        );
        self.passkeys.push(entry);
        self.new_passkey_rp.clear();
        self.new_passkey_username.clear();
        self.new_passkey_display.clear();
        self.show_add_passkey = false;
        self.save_vault();
    }

    pub fn import_tokens(&mut self) {
        let tokens = crate::import::import_tokens(&self.import_text);
        if tokens.is_empty() {
            self.import_error = Some("No valid tokens found".to_string());
        } else {
            self.import_error = None;
            self.tokens.extend(tokens);
            self.import_text.clear();
            self.show_import = false;
            self.save_vault();
        }
    }

    pub fn delete_token(&mut self, index: usize) {
        if index < self.tokens.len() {
            self.tokens.remove(index);
            self.delete_confirm = None;
            self.save_vault();
        }
    }

    pub fn delete_passkey(&mut self, index: usize) {
        if index < self.passkeys.len() {
            self.passkeys.remove(index);
            self.save_vault();
        }
    }
}

impl eframe::App for AuthenticatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.theme.apply_to_egui(ctx);

        if !self.is_unlocked {
            crate::ui::style::unlock_screen(ctx, self);
            return;
        }

        egui::TopBottomPanel::bottom("tab_bar").show(ctx, |ui| {
            crate::ui::style::tab_bar(ui, self);
        });

        let tab = self.current_tab;
        egui::CentralPanel::default().show(ctx, |ui| {
            match tab {
                Tab::Tokens => crate::ui::tokens::show(ui, self),
                Tab::Passkeys => crate::ui::passkey_ui::show(ui, self),
                Tab::Generator => crate::ui::password_ui::show(ui, self),
                Tab::Settings => crate::ui::settings_ui::show(ui, self),
            }
        });

        let show_add_token = self.show_add_token;
        let show_add_passkey = self.show_add_passkey;
        let show_import = self.show_import;

        if show_add_token {
            crate::ui::add_token::dialog(ctx, self);
        }
        if show_add_passkey {
            crate::ui::add_token::passkey_dialog(ctx, self);
        }
        if show_import {
            crate::ui::import_ui::dialog(ctx, self);
        }

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}
