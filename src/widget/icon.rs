//! Window icon and image helpers.
//!
//! All setters return `bool` and never panic: a failed decode or a
//! missing icon is reported as `false` rather than aborting the app.
//!
//! Uses only `fltk`'s built-in image decoders (`PngImage`, `IcoImage`)
//! plus `base64` for Base64 input. No `image` crate dependency.

use base64::{engine::general_purpose, Engine as _};
use fltk::{
    enums::{Align, Color, FrameType},
    frame::Frame,
    image::{IcoImage, PngImage, SharedImage},
    prelude::*,
    window::Window,
};

pub struct Icon;

impl Icon {
    /// Set a window icon from embedded PNG bytes.
    pub fn set_embd_png(window: &mut Window, data: &[u8]) -> bool {
        match PngImage::from_data(data) {
            Ok(img) => {
                window.set_icon(Some(img));
                true
            }
            Err(_) => false,
        }
    }

    /// Set a window icon from embedded ICO bytes.
    pub fn set_embd_ico(window: &mut Window, data: &[u8]) -> bool {
        match IcoImage::from_data(data) {
            Ok(img) => {
                window.set_icon(Some(img));
                true
            }
            Err(_) => false,
        }
    }

    /// Set a window icon from a Base64-encoded PNG string.
    pub fn set_base64_png(window: &mut Window, b64: &str) -> bool {
        let Ok(bytes) = general_purpose::STANDARD.decode(b64.trim()) else {
            return false;
        };
        Self::set_embd_png(window, &bytes)
    }

    /// Decode a Base64 PNG and display it in a `Frame`, scaled to fit.
    ///
    /// The frame's label is cleared and the image is centered. Returns
    /// `false` if decoding fails or the frame is too small to fit any
    /// image.
    pub fn show_base64_in_frame(frame: &mut Frame, b64: &str) -> bool {
        let Ok(bytes) = general_purpose::STANDARD.decode(b64.trim()) else {
            return false;
        };
        let Ok(mut img) = PngImage::from_data(&bytes) else {
            return false;
        };

        let (fw, fh) = (frame.w(), frame.h());
        if fw <= 0 || fh <= 0 {
            return false;
        }

        // Scale preserving aspect ratio, fit inside the frame.
        let (iw, ih) = (img.width(), img.height());
        if iw <= 0 || ih <= 0 {
            return false;
        }
        let scale = (fw as f64 / iw as f64).min(fh as f64 / ih as f64);
        let (nw, nh) = (
            (iw as f64 * scale).max(1.0) as i32,
            (ih as f64 * scale).max(1.0) as i32,
        );
        img.scale(nw, nh, true, true);

        frame.set_label("");
        // SharedImage::from_image 需要引用
        let Ok(shared) = SharedImage::from_image(&img) else {
            return false;
        };
        frame.set_image(Some(shared));
        // Align 中图片对齐常量名为 ImageMask
        frame.set_align(Align::Center | Align::ImageMask);
        frame.set_frame(FrameType::NoBox);
        frame.set_color(Color::White);
        true
    }
}