#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let auto_screenshots = std::env::args().any(|a| a == "--screenshots");
    let seed = if auto_screenshots {
        42
    } else {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(42, |d| d.as_nanos() as u64)
    };
    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_title("Galaga")
        .with_inner_size([800.0, 600.0])
        .with_min_inner_size([400.0, 300.0]);
    if let Ok(icon) = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")) {
        viewport = viewport.with_icon(icon);
    }
    eframe::run_native(
        "Galaga Rust",
        eframe::NativeOptions {
            viewport,
            renderer: eframe::Renderer::Glow,
            ..Default::default()
        },
        Box::new(move |cc| {
            Ok(Box::new(galaga_core::app::GalagaApp::new(
                cc,
                auto_screenshots,
                seed,
            )))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}
