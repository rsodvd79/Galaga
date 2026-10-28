use std::{
    io::Cursor,
    sync::{Arc, mpsc},
};

use eframe::egui::{ColorImage, Context, ScreenshotCallback, ViewportCommand};
use image::{DynamicImage, ImageFormat, RgbaImage, imageops::FilterType};

struct Captured {
    image: Arc<ColorImage>,
    phase: &'static str,
}

pub struct Screenshots {
    sender: mpsc::Sender<Captured>,
    receiver: mpsc::Receiver<Captured>,
    pub pending: bool,
}

impl Default for Screenshots {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender,
            receiver,
            pending: false,
        }
    }
}

impl Screenshots {
    pub fn request(&mut self, ctx: &Context, phase: &'static str) {
        if self.pending {
            return;
        }
        self.pending = true;
        let sender = self.sender.clone();
        let repaint = ctx.clone();
        ctx.send_viewport_cmd(ViewportCommand::Screenshot(ScreenshotCallback::new(
            move |image| {
                let _ = sender.send(Captured { image, phase });
                repaint.request_repaint();
            },
        )));
    }

    pub fn finish(&mut self) -> Option<Result<(), String>> {
        let captured = self.receiver.try_recv().ok()?;
        self.pending = false;
        Some(
            encode(&captured.image)
                .and_then(|bytes| export(&format!("galaga_{}.png", captured.phase), &bytes)),
        )
    }
}

fn encode(image: &ColorImage) -> Result<Vec<u8>, String> {
    let (width, height) = (image.size[0] as u32, image.size[1] as u32);
    if width == 0 || height == 0 {
        return Err("Empty framebuffer".into());
    }
    let bytes: Vec<_> = image
        .pixels
        .iter()
        .flat_map(|p| p.to_srgba_unmultiplied())
        .collect();
    let source = RgbaImage::from_raw(width, height, bytes).ok_or("Invalid framebuffer")?;
    let (crop_width, crop_height) = if u64::from(width) * 3 > u64::from(height) * 4 {
        (height * 4 / 3, height)
    } else {
        (width, width * 3 / 4)
    };
    let crop = image::imageops::crop_imm(
        &source,
        (width - crop_width) / 2,
        (height - crop_height) / 2,
        crop_width.max(1),
        crop_height.max(1),
    )
    .to_image();
    let output = DynamicImage::ImageRgba8(crop).resize_exact(800, 600, FilterType::Nearest);
    let mut buffer = Cursor::new(Vec::new());
    output
        .write_to(&mut buffer, ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    Ok(buffer.into_inner())
}

#[cfg(not(target_arch = "wasm32"))]
fn export(name: &str, bytes: &[u8]) -> Result<(), String> {
    let dir = std::path::Path::new("screenshots");
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let path = dir.join(name);
    std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
    println!("Screenshot saved: {}", path.display());
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn export(name: &str, bytes: &[u8]) -> Result<(), String> {
    use wasm_bindgen::JsCast;
    let download = || -> Result<(), wasm_bindgen::JsValue> {
        let parts = js_sys::Array::new();
        parts.push(&js_sys::Uint8Array::from(bytes));
        let options = web_sys::BlobPropertyBag::new();
        options.set_type("image/png");
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options)?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)?;
        let result = (|| {
            let document = web_sys::window()
                .and_then(|w| w.document())
                .ok_or("No document")?;
            let anchor = document
                .create_element("a")?
                .dyn_into::<web_sys::HtmlAnchorElement>()?;
            anchor.set_href(&url);
            anchor.set_download(name);
            let body = document.body().ok_or("No document body")?;
            body.append_child(&anchor)?;
            anchor.click();
            anchor.remove();
            Ok(())
        })();
        // Keep the URL alive until the browser has started processing the download.
        let cleanup = wasm_bindgen::closure::Closure::once_into_js(move || {
            let _ = web_sys::Url::revoke_object_url(&url);
        });
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cleanup.unchecked_ref(),
                1000,
            );
        }
        result
    };
    download().map_err(|e| format!("{e:?}"))
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static REQUESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn request_screenshot() {
    REQUESTED.set(true);
}

pub fn web_requested() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        REQUESTED.replace(false)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::Color32;

    #[test]
    fn png_crops_letterboxing_and_normalizes_hidpi() {
        let mut frame = ColorImage::filled([2400, 1200], Color32::BLACK);
        for y in 0..1200 {
            for x in 400..2000 {
                frame[(x, y)] = Color32::GREEN;
            }
        }
        let result = encode(&frame).unwrap();
        let decoded = image::load_from_memory(&result).unwrap().to_rgba8();
        assert_eq!(decoded.dimensions(), (800, 600));
        assert!(decoded.pixels().all(|p| p.0 == [0, 255, 0, 255]));
    }
}
