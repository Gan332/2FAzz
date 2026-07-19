fn main() -> eframe::Result {
    #[cfg(not(target_os = "android"))]
    env_logger::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Authenticator",
        options,
        Box::new(|cc| Ok(Box::new(authenticator::app::AuthenticatorApp::new(cc)))),
    )
}
