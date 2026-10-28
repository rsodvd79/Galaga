pub mod app;
pub mod audio;
pub mod entities;
pub mod game;
mod render;
mod screenshots;
mod storage;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|cc| {
                Ok(Box::new(app::GalagaApp::new(
                    cc,
                    false,
                    js_sys::Date::now() as u64,
                )))
            }),
        )
        .await
}
