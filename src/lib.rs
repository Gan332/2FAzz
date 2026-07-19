pub mod app;
pub mod import;
pub mod passkey;
pub mod password;
pub mod storage;
pub mod theme;
pub mod totp;
pub mod ui;

use std::sync::OnceLock;

pub static DATA_DIR: OnceLock<std::path::PathBuf> = OnceLock::new();

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let data_path = app.internal_data_path().unwrap_or_else(|| std::path::PathBuf::from("."));
    DATA_DIR.set(data_path.clone()).ok();

    let options = eframe::NativeOptions {
        android_app: Some(app),
        persistence_path: Some(data_path),
        ..Default::default()
    };

    eframe::run_native(
        "Authenticator",
        options,
        Box::new(|cc| Ok(Box::new(app::AuthenticatorApp::new(cc)))),
    )
    .unwrap();
}
